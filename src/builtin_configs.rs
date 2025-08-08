//! Built-in model configurations
//!
//! This module contains embedded configurations for known models,
//! allowing users to load models by ID without needing to manage
//! configuration files manually.

use crate::model_config::ModelConfig;
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Registry of built-in model configurations
///
/// These configurations are embedded at compile time and provide
/// known-good settings for popular models.
pub static BUILTIN_CONFIGS: Lazy<HashMap<&'static str, ModelConfig>> = Lazy::new(|| {
    let mut configs = HashMap::new();

    // Standard ANEMLL Qwen model
    if let Ok(config) = serde_json::from_str(ANEMLL_QWEN3_0_6B_CONFIG) {
        configs.insert("anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4", config);
    }

    // Typo-fixer model
    if let Ok(config) = serde_json::from_str(TYPO_FIXER_CONFIG) {
        configs.insert("mazhewitt/qwen-typo-fixer", config);
    }

    configs
});

/// Get built-in configuration for a model ID
pub fn get_builtin_config(model_id: &str) -> Option<ModelConfig> {
    BUILTIN_CONFIGS.get(model_id).cloned()
}

/// List all available built-in model IDs
pub fn list_builtin_models() -> Vec<&'static str> {
    BUILTIN_CONFIGS.keys().copied().collect()
}

/// Standard ANEMLL Qwen 3 0.6B model configuration
const ANEMLL_QWEN3_0_6B_CONFIG: &str = r#"{
  "model_info": {
    "model_id": "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4",
    "model_type": "qwen",
    "discovered_at": "2025-08-07T09:15:41.091815"
  },
  "shapes": {
    "batch_size": 64,
    "context_length": 512,
    "hidden_size": 1024,
    "vocab_size": 151936
  },
  "components": {
    "embeddings": {
  "file_path": "qwen_embeddings.mlmodelc",
      "inputs": {
        "input_ids": {
          "name": "input_ids",
          "shape": [1, 64],
          "data_type": "INT32"
        }
      },
      "outputs": {
        "hidden_states": {
          "name": "hidden_states", 
          "shape": [1, 64, 1024],
          "data_type": "FLOAT16"
        }
      },
      "functions": []
    },
    "ffn_prefill": {
  "file_path": "qwen_FFN_PF_lut8_chunk_01of01.mlmodelc",
      "inputs": {
        "hidden_states": {
          "name": "hidden_states",
          "shape": [1, 64, 1024],
          "data_type": "FLOAT16"
        },
        "position_ids": {
          "name": "position_ids",
          "shape": [64],
          "data_type": "INT32"
        },
        "causal_mask": {
          "name": "causal_mask",
          "shape": [1, 1, 64, 512],
          "data_type": "FLOAT16"
        },
        "current_pos": {
          "name": "current_pos",
          "shape": [1],
          "data_type": "INT32"
        }
      },
      "outputs": {
        "output_hidden_states": {
          "name": "output_hidden_states",
          "shape": [1, 1, 1024],
          "data_type": "FLOAT16"
        }
      },
      "functions": ["prefill", "infer"]
    },
    "ffn_infer": {
  "file_path": "qwen_FFN_PF_lut8_chunk_01of01.mlmodelc",
      "inputs": {
        "hidden_states": {
          "name": "hidden_states",
          "shape": [1, 1, 1024],
          "data_type": "FLOAT16"
        },
        "position_ids": {
          "name": "position_ids",
          "shape": [1],
          "data_type": "INT32"
        },
        "causal_mask": {
          "name": "causal_mask",
          "shape": [1, 1, 1, 512],
          "data_type": "FLOAT16"
        },
        "current_pos": {
          "name": "current_pos",
          "shape": [1],
          "data_type": "INT32"
        }
      },
      "outputs": {
        "output_hidden_states": {
          "name": "output_hidden_states",
          "shape": [1, 1, 1024],
          "data_type": "FLOAT16"
        }
      },
      "functions": ["infer"]
    },
    "lm_head": {
  "file_path": "qwen_lm_head_lut8.mlmodelc",
      "inputs": {
        "hidden_states": {
          "name": "hidden_states",
          "shape": [1, 1, 1024],
          "data_type": "FLOAT16"
        }
      },
      "outputs": {
        "logits": {
          "name": "logits",
          "shape": [1, 1, 151936],
          "data_type": "FLOAT32"
        }
      },
      "functions": []
    }
  },
  "naming": {
  "embeddings_pattern": null,
  "ffn_prefill_pattern": null,
  "lm_head_pattern": null
  }
}"#;

/// Typo-fixer model configuration
const TYPO_FIXER_CONFIG: &str = r#"{
  "model_info": {
    "model_id": "mazhewitt/qwen-typo-fixer",
    "path": "/Users/mazdahewitt/Library/Caches/candle-coreml/clean-mazhewitt--qwen-typo-fixer",
    "model_type": "qwen",
    "discovered_at": "2025-08-07T21:35:49.798615"
  },
  "shapes": {
    "batch_size": 64,
    "context_length": 256,
    "hidden_size": 1024,
    "vocab_size": 151669
  },
  "components": {
    "embeddings": {
      "file_path": "qwen-typo-fixer_embeddings.mlpackage",
      "inputs": {
        "input_ids": {
          "name": "input_ids",
          "shape": [1, 64],
          "data_type": "INT32"
        }
      },
      "outputs": {
        "hidden_states": {
          "name": "hidden_states", 
          "shape": [1, 64, 1024],
          "data_type": "FLOAT16"
        }
      },
      "functions": []
    },
    "ffn_prefill": {
      "file_path": "qwen-typo-fixer_FFN_PF_lut4_chunk_01of01.mlpackage",
      "inputs": {
        "hidden_states": {
          "name": "hidden_states",
          "shape": [1, 64, 1024],
          "data_type": "FLOAT16"
        },
        "position_ids": {
          "name": "position_ids",
          "shape": [1],
          "data_type": "INT32"
        },
        "causal_mask": {
          "name": "causal_mask",
          "shape": [1, 1, 64, 256],
          "data_type": "FLOAT16"
        },
        "current_pos": {
          "name": "current_pos",
          "shape": [1],
          "data_type": "INT32"
        }
      },
      "outputs": {
        "output_hidden_states": {
          "name": "output_hidden_states",
          "shape": [1, 1, 1024],
          "data_type": "FLOAT16"
        }
      },
      "functions": ["prefill"]
    },
    "ffn_infer": {
      "file_path": "qwen-typo-fixer_FFN_lut4_chunk_01of01.mlpackage",
      "inputs": {
        "hidden_states": {
          "name": "hidden_states",
          "shape": [1, 1, 1024],
          "data_type": "FLOAT16"
        },
        "position_ids": {
          "name": "position_ids",
          "shape": [1],
          "data_type": "INT32"
        },
        "causal_mask": {
          "name": "causal_mask",
          "shape": [1, 1, 1, 256],
          "data_type": "FLOAT16"
        },
        "current_pos": {
          "name": "current_pos",
          "shape": [1],
          "data_type": "INT32"
        }
      },
      "outputs": {
        "output_hidden_states": {
          "name": "output_hidden_states",
          "shape": [1, 1, 1024],
          "data_type": "FLOAT16"
        }
      },
      "functions": ["infer"]
    },
    "lm_head": {
      "file_path": "qwen-typo-fixer_lm_head_lut6.mlpackage",
      "inputs": {
        "hidden_states": {
          "name": "hidden_states",
          "shape": [1, 1, 1024],
          "data_type": "FLOAT16"
        }
      },
      "outputs": {
        "logits1": {
          "name": "logits1",
          "shape": [1, 1, 9496],
          "data_type": "FLOAT16"
        }
      },
      "functions": []
    }
  },
  "naming": {
  "embeddings_pattern": null,
  "lm_head_pattern": null,
  "ffn_prefill_pattern": null
  }
}"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_configs_load() {
        // Test that configurations parse correctly
        let anemll_config = get_builtin_config("anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4");
        assert!(anemll_config.is_some());

        // Test unknown model
        let unknown_config = get_builtin_config("unknown/model");
        assert!(unknown_config.is_none());
    }

    #[test]
    fn test_anemll_config_properties() {
        let config =
            get_builtin_config("anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4").unwrap();

        assert_eq!(config.shapes.batch_size, 64);
        assert_eq!(config.shapes.context_length, 512);
        assert_eq!(config.shapes.hidden_size, 1024);
        assert_eq!(config.shapes.vocab_size, 151936);

        // Check embeddings input shape (should be [1, 64] for standard ANEMLL)
        let embeddings_input = config.embeddings_input_shape().unwrap();
        assert_eq!(embeddings_input, &vec![1, 64]);
    }

    #[test]
    fn test_list_builtin_models() {
        let models = list_builtin_models();
        assert!(!models.is_empty());
        assert!(models.contains(&"anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4"));
    }

    #[test]
    fn test_configs_validation() {
        // Test that all built-in configs are valid
        for (model_id, config) in BUILTIN_CONFIGS.iter() {
            config.validate().unwrap_or_else(|e| {
                panic!("Built-in config for {model_id} is invalid: {e}");
            });
        }
    }
}
