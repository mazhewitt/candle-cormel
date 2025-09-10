//! Configuration types for CoreML models

use serde::{Deserialize, Serialize};

/// Configuration for CoreML models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Input tensor names in order (e.g., ["input_ids", "token_type_ids", "attention_mask"])
    pub input_names: Vec<String>,
    /// Output tensor name (e.g., "logits")
    pub output_name: String,
    /// Maximum sequence length
    pub max_sequence_length: usize,
    /// Vocabulary size
    pub vocab_size: usize,
    /// Model architecture name
    pub model_type: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            input_names: vec!["input_ids".to_string()],
            output_name: "logits".to_string(),
            max_sequence_length: 128,
            vocab_size: 32000,
            model_type: "coreml".to_string(),
        }
    }
}

impl Config {
    /// Create BERT-style config with input_ids, token_type_ids, and attention_mask
    pub fn bert_config(output_name: &str, max_seq_len: usize, vocab_size: usize) -> Self {
        Self {
            input_names: vec![
                "input_ids".to_string(),
                "token_type_ids".to_string(),
                "attention_mask".to_string(),
            ],
            output_name: output_name.to_string(),
            max_sequence_length: max_seq_len,
            vocab_size,
            model_type: "bert".to_string(),
        }
    }
}
