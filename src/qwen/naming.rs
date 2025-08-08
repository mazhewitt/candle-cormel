//! Model file naming patterns and discovery logic
//!
//! This module handles the various naming conventions used by different Qwen model variants,
//! including standard ANEMLL models and custom model formats.

use crate::ModelConfig;

/// Configuration for model file naming patterns
#[derive(Debug, Clone)]
pub struct ModelNamingConfig {
    /// Possible prefixes for embeddings model (e.g., ["qwen_", "bob-model_", "custom-"])
    pub embeddings_prefixes: Vec<String>,
    /// Suffix pattern for embeddings model (e.g., "embeddings.mlmodelc")
    pub embeddings_suffix: String,

    /// Possible prefixes for FFN model (e.g., ["qwen_FFN_PF_", "bob-model-ffn_"])
    pub ffn_prefixes: Vec<String>,
    /// Suffix pattern for FFN model (e.g., "_chunk_01of01.mlmodelc")
    pub ffn_suffix: String,

    /// Possible prefixes for LM head model (e.g., ["qwen_lm_head_", "bob-model-head_"])
    pub lm_head_prefixes: Vec<String>,
    /// Suffix pattern for LM head model (e.g., ".mlmodelc")
    pub lm_head_suffix: String,

    /// Supported file extensions, in order of preference
    pub supported_extensions: Vec<String>,
}

impl Default for ModelNamingConfig {
    fn default() -> Self {
        Self {
            embeddings_prefixes: vec!["qwen_".to_string()],
            embeddings_suffix: "embeddings.mlmodelc".to_string(),
            ffn_prefixes: vec!["qwen_FFN_PF_".to_string()],
            ffn_suffix: "_chunk_01of01.mlmodelc".to_string(),
            lm_head_prefixes: vec!["qwen_lm_head_".to_string()],
            lm_head_suffix: ".mlmodelc".to_string(),
            supported_extensions: vec![".mlpackage".to_string(), ".mlmodelc".to_string()],
        }
    }
}

impl ModelNamingConfig {
    /// Create a configuration for standard qwen models
    pub fn standard_qwen() -> Self {
        Self {
            embeddings_prefixes: vec!["qwen_".to_string()],
            embeddings_suffix: "embeddings.mlmodelc".to_string(),
            ffn_prefixes: vec!["qwen_FFN_PF_".to_string()],
            ffn_suffix: "_chunk_01of01.mlmodelc".to_string(),
            lm_head_prefixes: vec!["qwen_lm_head_".to_string()],
            lm_head_suffix: ".mlmodelc".to_string(),
            supported_extensions: vec![".mlpackage".to_string(), ".mlmodelc".to_string()],
        }
    }

    /// Create a custom configuration with user-defined patterns
    pub fn custom(base_prefix: &str, extension: &str) -> Self {
        Self {
            embeddings_prefixes: vec![
                format!("{base_prefix}_embeddings"),
                format!("{base_prefix}-embeddings"),
            ],
            embeddings_suffix: format!(".{extension}"),
            ffn_prefixes: vec![
                format!("{base_prefix}_FFN_PF_"),
                format!("{base_prefix}-ffn_"),
                format!("{base_prefix}_ffn_"),
            ],
            ffn_suffix: format!("_chunk_01of01.{extension}"),
            lm_head_prefixes: vec![
                format!("{base_prefix}_lm_head_"),
                format!("{base_prefix}-head_"),
                format!("{base_prefix}_head_"),
            ],
            lm_head_suffix: format!(".{extension}"),
            supported_extensions: vec![format!(".{extension}")],
        }
    }

    /// Create configuration for Bob's custom model example
    pub fn bobs_model() -> Self {
        Self::custom("bobs-qwen-model", "mlpackage")
    }

    /// Deprecated: explicit file paths are required; naming patterns are unused.
    pub fn from_model_config(_model_config: &ModelConfig) -> Self {
        Self::default()
    }
}
