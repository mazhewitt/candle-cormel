//! Mock objects and data for testing
#![allow(dead_code)]

use candle_coreml::Config;

/// Create a minimal valid Config for testing
pub fn create_mock_config() -> Config {
    Config {
        input_names: vec!["input_ids".to_string()],
        output_name: "logits".to_string(),
        max_sequence_length: 512,
        vocab_size: 151936,
        model_type: "test-model".to_string(),
    }
}

/// Sample token sequences for testing
pub mod test_data {
    pub const SHORT_SEQUENCE: &[i64] = &[1, 2, 3, 4, 5];
    pub const MEDIUM_SEQUENCE: &[i64] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    pub const SINGLE_TOKEN: i64 = 42;

    pub fn generate_sequence(length: usize) -> Vec<i64> {
        (0..length).map(|i| i as i64).collect()
    }
}
