#!/usr/bin/env python3
"""
CoreML Model Shape Discovery Tool

Extracts input/output shapes from ANEMLL model components.
This tool introspects CoreML models to generate configuration files
that can be used by the Rust candle-coreml library.

Usage:
    python discover_shapes.py --model-dir /path/to/model --output config.json
    python discover_shapes.py --scan-directory models/ --output-dir configs/
"""

import argparse
import json
import sys
from pathlib import Path
from typing import Dict, List, Optional, Tuple, Any
import re

try:
    import coremltools as ct
except ImportError:
    print("Error: coremltools not found. Install with: pip install coremltools", file=sys.stderr)
    sys.exit(1)


class ModelShapeDiscovery:
    """Discovers and analyzes shapes of CoreML model components."""
    
    def __init__(self, verbose: bool = False):
        self.verbose = verbose
        
    def discover_model_shapes(self, model_dir: Path) -> Dict[str, Any]:
        """Extract shapes from all model components in a directory."""
        if not model_dir.exists():
            raise FileNotFoundError(f"Model directory not found: {model_dir}")
            
        if self.verbose:
            print(f"🔍 Analyzing model directory: {model_dir}")
        
        # Find model components
        components = self._find_model_components(model_dir)
        if not components:
            raise ValueError(f"No CoreML model components found in {model_dir}")
        
        # Extract shapes from each component
        config = {
            "model_info": {
                "path": str(model_dir),
                "model_type": self._detect_model_type(model_dir),
                "discovered_at": self._get_timestamp(),
            },
            "shapes": {},
            "components": {},
            "naming": self._extract_naming_patterns(components)
        }
        
        # Process each component
        for component_type, model_path in components.items():
            if self.verbose:
                print(f"  📦 Processing {component_type}: {model_path.name}")
                
            try:
                component_config = self._extract_component_shapes(model_path)
                config["components"][component_type] = component_config
            except Exception as e:
                print(f"⚠️  Warning: Failed to process {component_type}: {e}", file=sys.stderr)
                continue
        
        # Derive overall model shapes
        config["shapes"] = self._derive_model_shapes(config["components"])
        
        # Validate configuration
        self._validate_config(config)
        
        return config
    
    def _find_model_components(self, model_dir: Path) -> Dict[str, Path]:
        """Find CoreML model files and classify them by component type."""
        components = {}
        
        # Look for .mlpackage and .mlmodelc files
        model_files = []
        for pattern in ["*.mlpackage", "*.mlmodelc"]:
            model_files.extend(model_dir.glob(pattern))
        
        if not model_files:
            # Look in subdirectories (some models are nested)
            for pattern in ["*/*.mlpackage", "*/*.mlmodelc"]:
                model_files.extend(model_dir.glob(pattern))
        
        # Classify files by component type
        for model_file in model_files:
            component_type = self._classify_component(model_file.name)
            if component_type:
                components[component_type] = model_file
                if self.verbose:
                    print(f"    Found {component_type}: {model_file.name}")
        
        return components
    
    def _classify_component(self, filename: str) -> Optional[str]:
        """Classify a model file by its component type based on naming patterns."""
        filename_lower = filename.lower()
        
        # Common patterns for ANEMLL models
        if "embedding" in filename_lower:
            return "embeddings"
        elif "ffn" in filename_lower and ("pf" in filename_lower or "prefill" in filename_lower):
            return "ffn_prefill"  
        elif "ffn" in filename_lower:
            return "ffn_infer"
        elif "lm_head" in filename_lower or "head" in filename_lower:
            return "lm_head"
        elif "prefill" in filename_lower:
            return "ffn_prefill"
        elif "infer" in filename_lower:
            return "ffn_infer"
        
        return None
    
    def _extract_component_shapes(self, model_path: Path) -> Dict[str, Any]:
        """Extract input/output shapes from a CoreML model component."""
        try:
            model = ct.models.MLModel(str(model_path))
            spec = model.get_spec()
            
            component_config = {
                "file_path": str(model_path),
                "inputs": {},
                "outputs": {},
                "functions": []
            }
            
            # Check if this is a multi-function model
            if hasattr(spec, 'pipelineSpec') and spec.pipelineSpec.models:
                # Multi-function model (like FFN with prefill/infer)
                for i, pipeline_model in enumerate(spec.pipelineSpec.models):
                    if hasattr(pipeline_model, 'mlProgram'):
                        function_name = f"function_{i}"
                        if hasattr(pipeline_model.mlProgram, 'functions'):
                            for func_name, func_spec in pipeline_model.mlProgram.functions.items():
                                function_name = func_name
                                break
                        component_config["functions"].append(function_name)
            
            # Extract input shapes
            for input_desc in spec.description.input:
                component_config["inputs"][input_desc.name] = self._extract_tensor_info(input_desc)
            
            # Extract output shapes
            for output_desc in spec.description.output:
                component_config["outputs"][output_desc.name] = self._extract_tensor_info(output_desc)
            
            return component_config
            
        except Exception as e:
            raise RuntimeError(f"Failed to load CoreML model {model_path}: {e}")
    
    def _extract_tensor_info(self, tensor_desc) -> Dict[str, Any]:
        """Extract shape and type information from a tensor description."""
        tensor_info = {
            "name": tensor_desc.name,
            "shape": [],
            "data_type": "unknown"
        }
        
        # Handle different tensor types
        if hasattr(tensor_desc.type, 'multiArrayType'):
            # MultiArray (most common)
            ma_type = tensor_desc.type.multiArrayType
            tensor_info["shape"] = [int(dim) for dim in ma_type.shape]
            tensor_info["data_type"] = self._get_data_type_name(ma_type.dataType)
        elif hasattr(tensor_desc.type, 'int64Type'):
            tensor_info["data_type"] = "INT64"
            tensor_info["shape"] = [1]  # Scalar
        elif hasattr(tensor_desc.type, 'doubleType'):
            tensor_info["data_type"] = "DOUBLE"
            tensor_info["shape"] = [1]  # Scalar
        elif hasattr(tensor_desc.type, 'stringType'):
            tensor_info["data_type"] = "STRING"
            tensor_info["shape"] = [1]  # Scalar
        
        return tensor_info
    
    def _get_data_type_name(self, data_type_enum) -> str:
        """Convert CoreML data type enum to string."""
        # Map CoreML data type constants to readable names
        type_map = {
            65600: "FLOAT32",    # kCVPixelFormatType_32BGRA
            65568: "FLOAT16",    # Half precision
            131104: "INT32",     # 32-bit integer
            131072: "INT64",     # 64-bit integer
            65552: "DOUBLE",     # Double precision
        }
        
        return type_map.get(data_type_enum, f"TYPE_{data_type_enum}")
    
    def _derive_model_shapes(self, components: Dict[str, Any]) -> Dict[str, Any]:
        """Derive overall model configuration from component shapes."""
        shapes = {
            "batch_size": 1,
            "context_length": 512,
            "hidden_size": 1024,
            "vocab_size": 151936
        }
        
        # Try to infer from embeddings component
        if "embeddings" in components:
            embeddings = components["embeddings"]
            if "input_ids" in embeddings["inputs"]:
                input_shape = embeddings["inputs"]["input_ids"]["shape"]
                if len(input_shape) >= 2:
                    shapes["batch_size"] = input_shape[0]
                    shapes["context_length"] = input_shape[1]
            
            if "hidden_states" in embeddings["outputs"]:
                output_shape = embeddings["outputs"]["hidden_states"]["shape"]
                if len(output_shape) >= 3:
                    shapes["hidden_size"] = output_shape[2]
        
        # Try to infer vocab size from LM head
        if "lm_head" in components:
            lm_head = components["lm_head"]
            # Look for logits outputs
            for output_name, output_info in lm_head["outputs"].items():
                if "logits" in output_name.lower():
                    output_shape = output_info["shape"]
                    if len(output_shape) >= 3:
                        # This might be a partial vocab (for multi-part outputs)
                        partial_vocab = output_shape[2]
                        # Count all logits outputs to get total vocab
                        total_vocab = sum(
                            info["shape"][2] for name, info in lm_head["outputs"].items()
                            if "logits" in name.lower() and len(info["shape"]) >= 3
                        )
                        shapes["vocab_size"] = total_vocab
                        break
        
        return shapes
    
    def _extract_naming_patterns(self, components: Dict[str, Path]) -> Dict[str, str]:
        """Extract naming patterns from discovered components."""
        patterns = {}
        
        for component_type, model_path in components.items():
            filename = model_path.name
            # Extract pattern by replacing specific parts with wildcards
            pattern = filename
            
            # Replace common variable parts with wildcards
            pattern = re.sub(r'lut\d+', 'lut*', pattern)  # lut4, lut6 -> lut*
            pattern = re.sub(r'chunk_\d+of\d+', 'chunk_*', pattern)  # chunk_01of01 -> chunk_*
            
            patterns[f"{component_type}_pattern"] = pattern
        
        return patterns
    
    def _detect_model_type(self, model_dir: Path) -> str:
        """Detect the model type from directory name or contents."""
        dir_name = model_dir.name.lower()
        
        if "qwen" in dir_name:
            return "qwen"
        elif "mistral" in dir_name:
            return "mistral"  
        elif "llama" in dir_name:
            return "llama"
        else:
            return "unknown"
    
    def _validate_config(self, config: Dict[str, Any]) -> None:
        """Validate the generated configuration."""
        required_components = ["embeddings", "lm_head"]
        missing_components = []
        
        for component in required_components:
            if component not in config["components"]:
                missing_components.append(component)
        
        if missing_components:
            print(f"⚠️  Warning: Missing required components: {missing_components}", file=sys.stderr)
        
        # Check for reasonable shape values
        shapes = config["shapes"]
        if shapes["batch_size"] <= 0 or shapes["batch_size"] > 1000:
            print(f"⚠️  Warning: Unusual batch_size: {shapes['batch_size']}", file=sys.stderr)
        
        if shapes["context_length"] <= 0 or shapes["context_length"] > 100000:
            print(f"⚠️  Warning: Unusual context_length: {shapes['context_length']}", file=sys.stderr)
        
        if self.verbose:
            print("✅ Configuration validation completed")
    
    def _get_timestamp(self) -> str:
        """Get current timestamp for metadata."""
        from datetime import datetime
        return datetime.now().isoformat()


def scan_directory_for_models(scan_dir: Path, output_dir: Path, verbose: bool = False) -> None:
    """Scan directory for ANEMLL models and generate configurations."""
    if not scan_dir.exists():
        print(f"Error: Scan directory not found: {scan_dir}", file=sys.stderr)
        return
    
    output_dir.mkdir(parents=True, exist_ok=True)
    discovery = ModelShapeDiscovery(verbose=verbose)
    
    # Look for potential model directories
    model_dirs = []
    for item in scan_dir.iterdir():
        if item.is_dir():
            # Check if directory contains .mlpackage or .mlmodelc files
            has_models = any(item.glob("*.mlpackage")) or any(item.glob("*.mlmodelc"))
            if has_models:
                model_dirs.append(item)
    
    if not model_dirs:
        print(f"No model directories found in {scan_dir}", file=sys.stderr)
        return
    
    print(f"📁 Found {len(model_dirs)} potential model directories")
    
    for model_dir in model_dirs:
        try:
            config = discovery.discover_model_shapes(model_dir)
            
            # Generate output filename
            output_filename = f"{model_dir.name}.json"
            output_path = output_dir / output_filename
            
            # Save configuration
            with open(output_path, 'w') as f:
                json.dump(config, f, indent=2)
            
            print(f"✅ Generated config: {output_path}")
            
            # Print summary
            shapes = config["shapes"]
            components = list(config["components"].keys())
            print(f"   Model: {config['model_info']['model_type']}")
            print(f"   Shapes: batch={shapes['batch_size']}, ctx={shapes['context_length']}, hidden={shapes['hidden_size']}")
            print(f"   Components: {', '.join(components)}")
            print()
            
        except Exception as e:
            print(f"❌ Failed to process {model_dir.name}: {e}", file=sys.stderr)
            continue


def main():
    parser = argparse.ArgumentParser(
        description="Discover shapes from CoreML ANEMLL models",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Discover shapes for a single model
  python discover_shapes.py --model-dir models/qwen-typo-fixer-ane --output config.json
  
  # Scan directory and generate configs for all models
  python discover_shapes.py --scan-directory models/ --output-dir configs/
  
  # Verbose output with detailed analysis
  python discover_shapes.py --model-dir models/qwen-typo-fixer-ane --output config.json --verbose
        """
    )
    
    # Main operation mode
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--model-dir", type=Path, 
                      help="Path to model directory to analyze")
    group.add_argument("--scan-directory", type=Path,
                      help="Scan directory for models and generate configs")
    
    # Output options
    parser.add_argument("--output", type=Path,
                       help="Output JSON file (for --model-dir mode)")
    parser.add_argument("--output-dir", type=Path, 
                       help="Output directory for configs (for --scan-directory mode)")
    
    # Options
    parser.add_argument("--verbose", "-v", action="store_true",
                       help="Verbose output with detailed analysis")
    
    args = parser.parse_args()
    
    try:
        if args.model_dir:
            # Single model analysis
            if not args.output:
                parser.error("--output is required when using --model-dir")
                
            discovery = ModelShapeDiscovery(verbose=args.verbose)
            config = discovery.discover_model_shapes(args.model_dir)
            
            # Save configuration
            with open(args.output, 'w') as f:
                json.dump(config, f, indent=2)
            
            print(f"✅ Configuration saved to: {args.output}")
            
            # Print summary
            shapes = config["shapes"]
            components = list(config["components"].keys())
            print(f"📊 Model Summary:")
            print(f"   Type: {config['model_info']['model_type']}")
            print(f"   Batch Size: {shapes['batch_size']}")
            print(f"   Context Length: {shapes['context_length']}")
            print(f"   Hidden Size: {shapes['hidden_size']}")
            print(f"   Vocab Size: {shapes['vocab_size']}")
            print(f"   Components: {', '.join(components)}")
            
        else:
            # Batch directory scanning
            if not args.output_dir:
                parser.error("--output-dir is required when using --scan-directory")
                
            scan_directory_for_models(args.scan_directory, args.output_dir, args.verbose)
            
    except KeyboardInterrupt:
        print("\n⏹️  Operation cancelled by user", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"❌ Error: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()