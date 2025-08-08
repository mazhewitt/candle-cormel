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

    /// Create ModelNamingConfig from a ModelConfig
    pub fn from_model_config(model_config: &ModelConfig) -> Self {
        // Convert glob patterns from ModelConfig to prefix/suffix format expected by find_model_file_with_config

        // For embeddings pattern like "*_embeddings.mlmodelc" -> prefix: ["qwen_"], suffix: "embeddings.mlmodelc"
        let embeddings_prefixes = if model_config.naming.embeddings_pattern.starts_with('*') {
            // Standard ANEMLL pattern
            vec!["qwen_".to_string()]
        } else {
            // Direct pattern like "qwen-typo-fixer_embeddings.mlpackage"
            let base_name = model_config
                .naming
                .embeddings_pattern
                .split('_')
                .next()
                .unwrap_or("qwen");
            vec![format!("{}_", base_name)]
        };

        let embeddings_suffix = if model_config.naming.embeddings_pattern.contains('_') {
            // Extract suffix after first underscore: "*_embeddings.mlmodelc" -> "embeddings.mlmodelc"  
            // OR "qwen-typo-fixer_embeddings.mlpackage" -> "embeddings.mlpackage"
            model_config
                .naming
                .embeddings_pattern
                .split_once('_')
                .map(|(_, suffix)| suffix.to_string())
                .unwrap_or_else(|| model_config.naming.embeddings_pattern.clone())
        } else {
            model_config.naming.embeddings_pattern.clone()
        };

        // For FFN: Use the actual pattern from model config instead of hardcoded lut8
        // Pattern like "qwen-typo-fixer_FFN_PF_lut*_chunk_*.mlpackage"
        let (ffn_prefixes, ffn_suffix) = if let Some(ffn_pattern) = &model_config.naming.ffn_prefill_pattern {
            let ffn_prefixes = if ffn_pattern.starts_with('*') {
                vec!["qwen_".to_string()]
            } else {
                let base_name = ffn_pattern
                    .split('_')
                    .next()
                    .unwrap_or("qwen");
                vec![format!("{}_", base_name)]
            };

            let ffn_suffix = if let Some((_, suffix)) = ffn_pattern.split_once('_') {
                // "qwen-typo-fixer_FFN_PF_lut*_chunk_*.mlpackage" -> "FFN_PF_lut*_chunk_*.mlpackage"
                suffix.to_string()
            } else {
                ffn_pattern.clone()
            };
            (ffn_prefixes, ffn_suffix)
        } else {
            // Fallback to default if no FFN pattern specified
            (vec!["qwen_".to_string()], "FFN_PF_lut8_chunk_01of01.mlmodelc".to_string())
        };

        // For LM head: Use the actual pattern from model config
        // Pattern like "qwen-typo-fixer_lm_head_lut*.mlpackage"  
        let lm_head_prefixes = if model_config.naming.lm_head_pattern.starts_with('*') {
            vec!["qwen_".to_string()]
        } else {
            let base_name = model_config
                .naming
                .lm_head_pattern
                .split('_')
                .next()
                .unwrap_or("qwen");
            vec![format!("{}_", base_name)]
        };

        let lm_head_suffix = if let Some((_, suffix)) = model_config.naming.lm_head_pattern.split_once('_') {
            // "qwen-typo-fixer_lm_head_lut*.mlpackage" -> "lm_head_lut*.mlpackage"
            suffix.to_string()
        } else {
            model_config.naming.lm_head_pattern.clone()
        };

        Self {
            embeddings_prefixes,
            embeddings_suffix,
            ffn_prefixes,
            ffn_suffix,
            lm_head_prefixes,
            lm_head_suffix,
            supported_extensions: vec![".mlpackage".to_string(), ".mlmodelc".to_string()],
        }
    }
}
