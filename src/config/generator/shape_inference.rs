//! Generic shape inference from CoreML components
//!
//! Infers model dimensions (batch size, hidden size, etc.) from actual tensor configurations

use super::schema_extractor::SchemaExtractor;
use crate::config::model::{ComponentConfig, ShapeConfig};
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use tracing::debug;

pub struct ShapeInference;

impl Default for ShapeInference {
    fn default() -> Self {
        Self::new()
    }
}

impl ShapeInference {
    pub fn new() -> Self {
        Self
    }

    /// Compute overall shape configuration from components (enhanced with metadata-driven detection)
    /// Returns an error if components have insufficient tensor metadata
    pub fn infer_shapes_with_schema_extractor(
        &self,
        components: &HashMap<String, ComponentConfig>,
        schema_extractor: &SchemaExtractor,
    ) -> Result<ShapeConfig, anyhow::Error> {
        // Strict validation: require sufficient tensor metadata
        self.validate_tensor_metadata(components)?;

        let batch_size = self.infer_batch_size(components);
        let hidden_size = self.infer_hidden_size(components);
        let context_length = self.infer_context_length(components);
        let vocab_size = self.infer_vocab_size_with_chunking(components, schema_extractor);

        debug!(
            "📊 Inferred shapes (validated): batch={}, context={}, hidden={}, vocab={}",
            batch_size, context_length, hidden_size, vocab_size
        );

        Ok(ShapeConfig {
            batch_size,
            context_length,
            hidden_size,
            vocab_size,
        })
    }

    /// Compute overall shape configuration from components (legacy approach)
    /// Returns an error if components have insufficient tensor metadata
    pub fn infer_shapes(
        &self,
        components: &HashMap<String, ComponentConfig>,
    ) -> Result<ShapeConfig, anyhow::Error> {
        // Strict validation on legacy path as well
        self.validate_tensor_metadata(components)?;

        let batch_size = self.infer_batch_size(components);
        let hidden_size = self.infer_hidden_size(components);
        let context_length = self.infer_context_length(components);
        let vocab_size = self.infer_vocab_size(components);

        debug!(
            "📊 Inferred shapes (validated): batch={}, context={}, hidden={}, vocab={}",
            batch_size, context_length, hidden_size, vocab_size
        );

        Ok(ShapeConfig {
            batch_size,
            context_length,
            hidden_size,
            vocab_size,
        })
    }

    /// Infer batch size from the smallest batch dimension across all components
    fn infer_batch_size(&self, components: &HashMap<String, ComponentConfig>) -> usize {
        let mut batch_sizes = Vec::new();

        for component in components.values() {
            for tensor in component.inputs.values().chain(component.outputs.values()) {
                if !tensor.shape.is_empty() {
                    batch_sizes.push(tensor.shape[0]);
                }
            }
        }

        batch_sizes.into_iter().min().unwrap_or(1)
    }

    /// Infer hidden size from hidden_states tensors, ignoring logits. Falls back to heuristic if needed.
    fn infer_hidden_size(&self, components: &HashMap<String, ComponentConfig>) -> usize {
        let mut from_hidden_states = Vec::new();
        let mut heuristic = Vec::new();

        for component in components.values() {
            // Prefer tensors explicitly named "hidden_states"
            for (name, tensor) in component.inputs.iter().chain(component.outputs.iter()) {
                // Skip any logits-like tensors
                let lname = name.to_lowercase();
                let is_logits = lname.starts_with("logits");

                if tensor.shape.len() >= 3 {
                    let feat = tensor.shape[2];
                    if name == "hidden_states" {
                        from_hidden_states.push(feat);
                    } else if !is_logits {
                        heuristic.push(feat);
                    }
                } else if tensor.shape.len() == 2 {
                    // Some manifests might flatten features as [1, hidden]
                    let feat = tensor.shape[1];
                    if name == "hidden_states" {
                        from_hidden_states.push(feat);
                    } else if !is_logits && feat > 100 {
                        heuristic.push(feat);
                    }
                }
            }
        }

        if let Some(max_hidden) = from_hidden_states.into_iter().max() {
            return max_hidden;
        }
        heuristic.into_iter().max().unwrap_or(1024)
    }

    /// Infer context/sequence length from sequence dimensions
    fn infer_context_length(&self, components: &HashMap<String, ComponentConfig>) -> usize {
        let mut seq_lengths = Vec::new();

        for component in components.values() {
            for tensor in component.inputs.values().chain(component.outputs.values()) {
                if tensor.shape.len() >= 2 && tensor.shape[1] > 1 {
                    // 2D+ tensors: sequence dimension is usually index 1
                    seq_lengths.push(tensor.shape[1]);
                }
                // Also check 4D tensors (e.g., attention masks)
                if tensor.shape.len() >= 4 {
                    seq_lengths.push(tensor.shape[3]);
                }
            }
        }

        seq_lengths.into_iter().max().unwrap_or(256)
    }

    /// Infer vocabulary size with chunked logits support (typo-fixer model pattern)
    fn infer_vocab_size_with_chunking(
        &self,
        components: &HashMap<String, ComponentConfig>,
        schema_extractor: &SchemaExtractor,
    ) -> usize {
        // First, try to find the lm_head component and use schema extractor's logic
        for (name, component) in components {
            if name == "lm_head" || name.contains("lm_head") {
                if let Some(vocab_size) =
                    schema_extractor.calculate_vocab_size_from_logits(&component.outputs)
                {
                    debug!(
                        "📊 Using chunked logits vocab size calculation: {}",
                        vocab_size
                    );
                    return vocab_size;
                }
            }
        }

        // Fallback to legacy logic
        debug!("📊 Using legacy vocab size detection");
        self.infer_vocab_size(components)
    }

    /// Infer vocabulary size from the largest output dimension (legacy)
    fn infer_vocab_size(&self, components: &HashMap<String, ComponentConfig>) -> usize {
        let mut output_sizes = Vec::new();

        for component in components.values() {
            for tensor in component.outputs.values() {
                if let Some(&last_dim) = tensor.shape.last() {
                    if last_dim > 1000 {
                        // Likely a vocabulary or class dimension
                        output_sizes.push(last_dim);
                    }
                }
            }
        }

        output_sizes.into_iter().max().unwrap_or(30000)
    }

    /// Analyze component characteristics for debugging
    pub fn analyze_components(
        &self,
        components: &HashMap<String, ComponentConfig>,
    ) -> ComponentAnalysis {
        let mut analysis = ComponentAnalysis::default();

        for (name, component) in components {
            let comp_analysis = self.analyze_single_component(name, component);
            analysis.components.insert(name.clone(), comp_analysis);
        }

        analysis.total_components = components.len();
        analysis.function_based_components = components
            .values()
            .filter(|c| !c.functions.is_empty())
            .count();
        analysis.multi_function_components = components
            .values()
            .filter(|c| c.functions.len() > 1)
            .count();

        analysis
    }

    fn analyze_single_component(
        &self,
        name: &str,
        component: &ComponentConfig,
    ) -> SingleComponentAnalysis {
        let input_shapes: Vec<Vec<usize>> =
            component.inputs.values().map(|t| t.shape.clone()).collect();
        let output_shapes: Vec<Vec<usize>> = component
            .outputs
            .values()
            .map(|t| t.shape.clone())
            .collect();

        let max_input_dim = input_shapes.iter().flatten().max().copied().unwrap_or(0);
        let max_output_dim = output_shapes.iter().flatten().max().copied().unwrap_or(0);

        SingleComponentAnalysis {
            name: name.to_string(),
            input_count: component.inputs.len(),
            output_count: component.outputs.len(),
            function_count: component.functions.len(),
            input_shapes,
            output_shapes,
            max_input_dimension: max_input_dim,
            max_output_dimension: max_output_dim,
        }
    }

    /// Validate that components have sufficient tensor metadata for shape inference
    /// Returns an error with actionable guidance if metadata is insufficient
    fn validate_tensor_metadata(
        &self,
        components: &HashMap<String, ComponentConfig>,
    ) -> Result<()> {
        let mut empty_components = Vec::new();
        let mut components_with_issues = Vec::new();

        for (name, component) in components {
            // Check for completely empty tensor maps
            if component.inputs.is_empty() && component.outputs.is_empty() {
                empty_components.push(name.clone());
                continue;
            }

            // Check for components with tensors but no shape information
            let has_valid_shapes = component
                .inputs
                .values()
                .chain(component.outputs.values())
                .any(|tensor| !tensor.shape.is_empty() && tensor.shape.iter().all(|&dim| dim > 0));

            if !has_valid_shapes {
                components_with_issues.push(name.clone());
            }
        }

        // Fail fast with clear error messages
        if !empty_components.is_empty() {
            return Err(anyhow!(
                "Configuration generation failed: Components have empty tensor metadata.\n\
                 \n\
                 🔍 Components with empty tensor maps: {:?}\n\
                 \n\
                 💡 This typically indicates one of these issues:\n\
                    1. CoreML model files lack proper metadata (model.mlmodel missing or corrupt)\n\
                    2. Model packages are incomplete (.mlpackage structure is invalid)\n\
                    3. Metadata extraction failed during parsing\n\
                 \n\
                 🛠️  Possible solutions:\n\
                    1. Re-download the model from the original source\n\
                    2. Verify .mlpackage directory structure contains Data/com.apple.CoreML/model.mlmodel\n\
                    3. Check model compatibility with this version of candle-coreml\n\
                    4. For typo-fixer models: ensure using the correct coreml variant from HuggingFace\n\
                 \n\
                 📝 Expected tensor information:\n\
                    - Embeddings: input_ids → hidden_states\n\
                    - FFN: hidden_states + causal_mask → hidden_states\n\
                    - LM Head: hidden_states → logits (potentially chunked)",
                empty_components
            ));
        }

        if !components_with_issues.is_empty() {
            return Err(anyhow!(
                "Configuration generation failed: Components have invalid tensor shape information.\n\
                 \n\
                 🔍 Components with shape issues: {:?}\n\
                 \n\
                 💡 These components have tensor information but with invalid shapes (empty or zero dimensions).\n\
                 \n\
                 🛠️  This suggests corrupted model metadata. Try re-downloading the model.",
                components_with_issues
            ));
        }

        // Additional validation for specific model patterns
        self.validate_model_specific_requirements(components)?;

        debug!(
            "✅ Tensor metadata validation passed for {} components",
            components.len()
        );
        Ok(())
    }

    /// Validate model-specific requirements (e.g., typo-fixer needs proper vocab size)
    fn validate_model_specific_requirements(
        &self,
        components: &HashMap<String, ComponentConfig>,
    ) -> Result<()> {
        // Detect if this looks like a typo-fixer model based on filenames
        let looks_like_typo_fixer = components
            .keys()
            .any(|name| name.contains("typo") || name.contains("fixer"))
            || components.values().any(|comp| {
                comp.file_path
                    .as_ref()
                    .map(|path| path.contains("typo-fixer"))
                    .unwrap_or(false)
            });

        if looks_like_typo_fixer {
            // For typo-fixer models, validate we can extract proper vocab size from LM head
            let lm_head = components.get("lm_head");
            if let Some(lm_head) = lm_head {
                let has_logits = lm_head.outputs.keys().any(|k| k.contains("logits"));
                let logits_total_size: usize = lm_head
                    .outputs
                    .iter()
                    .filter(|(name, _)| name.contains("logits"))
                    .map(|(_, tensor)| tensor.shape.last().copied().unwrap_or(0))
                    .sum();

                if !has_logits {
                    return Err(anyhow!(
                        "Typo-fixer model validation failed: LM head component lacks logits outputs.\n\
                         \n\
                         🔍 LM head outputs found: {:?}\n\
                         \n\
                         💡 Typo-fixer models require chunked logits outputs (logits_0, logits_1, etc.)\n\
                         🛠️  Ensure you're using the correct typo-fixer coreml model variant.",
                        lm_head.outputs.keys().collect::<Vec<_>>()
                    ));
                }

                // Typo-fixer should have vocab size around 151,669
                if logits_total_size > 0 && logits_total_size < 100000 {
                    return Err(anyhow!(
                        "Typo-fixer model validation failed: Vocabulary size {} is too small.\n\
                         \n\
                         💡 Expected vocab size ≥ 100,000 for typo-fixer (typically 151,669)\n\
                         🔍 Detected logits total size: {}\n\
                         \n\
                         🛠️  This suggests model metadata extraction issues or wrong model variant.",
                        logits_total_size, logits_total_size
                    ));
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct ComponentAnalysis {
    pub total_components: usize,
    pub function_based_components: usize,
    pub multi_function_components: usize,
    pub components: HashMap<String, SingleComponentAnalysis>,
}

#[derive(Debug)]
pub struct SingleComponentAnalysis {
    pub name: String,
    pub input_count: usize,
    pub output_count: usize,
    pub function_count: usize,
    pub input_shapes: Vec<Vec<usize>>,
    pub output_shapes: Vec<Vec<usize>>,
    pub max_input_dimension: usize,
    pub max_output_dimension: usize,
}
