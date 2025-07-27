//! End-to-end Qwen model implementation
//! 
//! This demonstrates how to run the complete Qwen pipeline:
//! 1. Embeddings: input_ids -> hidden_states  
//! 2. FFN: hidden_states + position_ids + causal_mask + current_pos -> output_hidden_states
//! 3. LM Head: hidden_states -> logits1..logits16 (then combine)

use anyhow::Result;
use candle_core::{Device, Tensor};
use candle_coreml::{Config as CoreMLConfig, CoreMLModel, CoreMLState};
use std::collections::HashMap;
use std::path::Path;

const QWEN_VOCAB_SIZE: usize = 151936;
const HIDDEN_SIZE: usize = 1024; // From metadata: 1 × 1 × 1024

/// Complete Qwen model with all components
pub struct QwenModel {
    embeddings: CoreMLModel,
    ffn: CoreMLModel,
    lm_head: CoreMLModel,
    device: Device,
    ffn_state: CoreMLState,
}

impl QwenModel {
    /// Load all Qwen components from the local directory
    pub fn load_local(base_path: &Path) -> Result<Self> {
        let device = Device::Cpu;
        
        // Load embeddings
        let embeddings_path = base_path.join("qwen_embeddings.mlmodelc");
        let embeddings_config = CoreMLConfig {
            input_names: vec!["input_ids".to_string()],
            output_name: "hidden_states".to_string(),
            max_sequence_length: 512,
            vocab_size: QWEN_VOCAB_SIZE,
            model_type: "qwen-embeddings".to_string(),
        };
        let embeddings = CoreMLModel::load_from_file(&embeddings_path, &embeddings_config)?;
        
        // Load FFN
        let ffn_path = base_path.join("qwen_FFN_PF_lut8_chunk_01of01.mlmodelc");
        let ffn_config = CoreMLConfig {
            input_names: vec![
                "hidden_states".to_string(), 
                "position_ids".to_string(), 
                "causal_mask".to_string(), 
                "current_pos".to_string()
            ],
            output_name: "output_hidden_states".to_string(),
            max_sequence_length: 512,
            vocab_size: HIDDEN_SIZE,
            model_type: "qwen-ffn".to_string(),
        };
        let ffn = CoreMLModel::load_from_file(&ffn_path, &ffn_config)?;
        
        // Load LM head (with multiple outputs)
        let lm_head_path = base_path.join("qwen_lm_head_lut8.mlmodelc");
        let lm_head_config = CoreMLConfig {
            input_names: vec!["hidden_states".to_string()],
            output_name: "logits1".to_string(), // We'll extract all outputs manually
            max_sequence_length: 512,
            vocab_size: QWEN_VOCAB_SIZE,
            model_type: "qwen-lm-head".to_string(),
        };
        let lm_head = CoreMLModel::load_from_file(&lm_head_path, &lm_head_config)?;
        
        // Create state for FFN model
        let ffn_state = ffn.make_state()?;
        
        Ok(Self {
            embeddings,
            ffn,
            lm_head,
            device,
            ffn_state,
        })
    }
    
    /// Run complete forward pass for a single token generation
    pub fn forward(&mut self, input_ids: &[i64], position: usize) -> Result<Tensor> {
        let seq_len = input_ids.len();
        
        // Step 1: Embeddings
        let input_tensor = Tensor::from_vec(
            input_ids.to_vec(),
            (1, seq_len),
            &self.device,
        )?;
        
        let hidden_states = self.embeddings.forward(&[&input_tensor])?;
        println!("Embeddings output shape: {:?}", hidden_states.dims());
        
        // Step 2: Create FFN inputs
        let position_ids = Tensor::from_vec(
            vec![position as i64],
            (1,),
            &self.device,
        )?;
        
        let current_pos = Tensor::from_vec(
            vec![position as i64],
            (1,),
            &self.device,
        )?;
        
        // Create causal mask (simple version - all zeros for now)
        let causal_mask = Tensor::zeros(
            (1, 1, seq_len, 512),
            candle_core::DType::F32,
            &self.device,
        )?;
        
        // Step 3: FFN forward pass with state
        let output_hidden_states = self.ffn.predict_with_state(&[
            &hidden_states, 
            &position_ids, 
            &causal_mask, 
            &current_pos
        ], &mut self.ffn_state)?;
        println!("FFN output shape: {:?}", output_hidden_states.dims());
        
        // Step 4: LM Head with multiple outputs
        let lm_head_outputs = self.extract_lm_head_outputs(&output_hidden_states)?;
        
        // Step 5: Combine the 16 logits chunks
        let combined_logits = self.combine_logits_chunks(lm_head_outputs)?;
        println!("Combined logits shape: {:?}", combined_logits.dims());
        
        Ok(combined_logits)
    }
    
    /// Extract all 16 logits outputs from LM head
    fn extract_lm_head_outputs(&self, hidden_states: &Tensor) -> Result<HashMap<String, Tensor>> {
        // Use forward_all to get all outputs (this uses forward_all_impl internally)
        self.lm_head.forward_all(&[hidden_states])
            .map_err(|e| anyhow::Error::msg(format!("LM head forward failed: {}", e)))
    }
    
    /// Combine the 16 logits chunks into a single vocabulary tensor
    fn combine_logits_chunks(&self, outputs: HashMap<String, Tensor>) -> Result<Tensor> {
        // Extract logits1 through logits16 and concatenate
        let mut logits_chunks = Vec::new();
        
        for i in 1..=16 {
            let key = format!("logits{}", i);
            if let Some(chunk) = outputs.get(&key) {
                logits_chunks.push(chunk.clone());
            } else {
                return Err(anyhow::Error::msg(format!("Missing logits chunk: {}", key)));
            }
        }
        
        // Concatenate along the vocabulary dimension (last dimension)
        let combined = Tensor::cat(&logits_chunks, 2)?;
        Ok(combined)
    }
    
    /// Generate next token using argmax
    pub fn generate_next_token(&mut self, input_ids: &[i64], position: usize) -> Result<i64> {
        let logits = self.forward(input_ids, position)?;
        
        // Get logits for the last token
        let last_token_logits = logits.get(0)?.get(input_ids.len() - 1)?;
        
        // Argmax to get next token
        let logits_vec = last_token_logits.to_vec1::<f32>()?;
        let next_token = logits_vec
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i as i64)
            .unwrap();
        
        Ok(next_token)
    }
}

fn main() -> Result<()> {
    let base_path = Path::new("/Users/mazdahewitt/projects/candle-coreml/qwen-model");
    
    println!("🚀 Loading Qwen model components...");
    let mut model = QwenModel::load_local(base_path)?;
    println!("✅ All components loaded successfully!");
    
    // Test with simple input
    let input_ids = vec![1i64]; // Simple test token
    let position = 0;
    
    println!("🧪 Running forward pass with input_ids: {:?}", input_ids);
    
    match model.forward(&input_ids, position) {
        Ok(logits) => {
            println!("✅ Forward pass successful!");
            println!("📊 Final logits shape: {:?}", logits.dims());
            
            // Generate next token
            match model.generate_next_token(&input_ids, position) {
                Ok(next_token) => {
                    println!("🎯 Generated next token: {}", next_token);
                }
                Err(e) => {
                    println!("❌ Token generation failed: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Forward pass failed: {}", e);
        }
    }
    
    Ok(())
}