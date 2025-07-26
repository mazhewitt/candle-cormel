//! Debug components to test each step of the pipeline
//! This will be used to compare with Python reference outputs

use crate::{CoreMLModel, Config as CoreMLConfig};
use anyhow::{Result, Error as E};
use candle_core::{Device, Tensor, IndexOp};
use std::path::Path;
use tokenizers::Tokenizer;

pub struct PipelineDebugger {
    pub embeddings: CoreMLModel,
    pub ffn: CoreMLModel,
    pub lm_head: CoreMLModel,
    pub tokenizer: Tokenizer,
    pub device: Device,
}

impl PipelineDebugger {
    pub fn new<P: AsRef<Path>>(model_dir: P) -> Result<Self> {
        let model_dir = model_dir.as_ref();
        let device = Device::Cpu;
        
        // Load tokenizer
        let tokenizer_path = model_dir.join("tokenizer.json");
        let tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| E::msg(format!("Failed to load tokenizer: {}", e)))?;
        
        // Load embeddings
        let embeddings_config = CoreMLConfig {
            input_names: vec!["input_ids".to_string()],
            output_name: "hidden_states".to_string(),
            max_sequence_length: 512,
            vocab_size: 151936,
            model_type: "qwen-embeddings".to_string(),
        };
        let embeddings_path = model_dir.join("qwen_embeddings.mlmodelc");
        let embeddings = CoreMLModel::load_from_file(&embeddings_path, &embeddings_config)?;
        
        // Load FFN
        let ffn_config = CoreMLConfig {
            input_names: vec![
                "hidden_states".to_string(),
                "position_ids".to_string(),
                "current_pos".to_string(),
                "causal_mask".to_string(),
            ],
            output_name: "output_hidden_states".to_string(),
            max_sequence_length: 512,
            vocab_size: 1024,
            model_type: "qwen-ffn".to_string(),
        };
        let ffn_path = model_dir.join("qwen_FFN_PF_lut6_chunk_01of01.mlmodelc");
        let ffn = CoreMLModel::load_from_file(&ffn_path, &ffn_config)?;
        
        // Load LM head
        let lm_head_config = CoreMLConfig {
            input_names: vec!["hidden_states".to_string()],
            output_name: "logits1".to_string(),
            max_sequence_length: 512,
            vocab_size: 151936,
            model_type: "qwen-lm-head".to_string(),
        };
        let lm_head_path = model_dir.join("qwen_lm_head_lut6.mlmodelc");
        let lm_head = CoreMLModel::load_from_file(&lm_head_path, &lm_head_config)?;
        
        Ok(Self {
            embeddings,
            ffn,
            lm_head,
            tokenizer,
            device,
        })
    }
    
    pub fn test_tokenization(&self) -> Result<Vec<i64>> {
        println!("============================================================");
        println!("STEP 1: TOKENIZATION (RUST)");
        println!("============================================================");
        
        let test_input = "Hello world";
        let chat_template = format!("<|im_start|>user\n{}<|im_end|>\n<|im_start|>assistant\n", test_input);
        
        println!("Input text: {:?}", test_input);
        println!("Chat template: {:?}", chat_template);
        
        let encoding = self.tokenizer.encode(chat_template, false).unwrap();
        let tokens: Vec<i64> = encoding.get_ids().iter().map(|&id| id as i64).collect();
        
        println!("Tokens: {:?}", tokens);
        println!("Token count: {}", tokens.len());
        
        println!("Token breakdown:");
        for (i, &token_id) in tokens.iter().enumerate() {
            let token_text = self.tokenizer.decode(&[token_id as u32], false).unwrap();
            println!("  {}: {} -> {:?}", i, token_id, token_text);
        }
        
        Ok(tokens)
    }
    
    pub fn test_embeddings(&self, token: i64) -> Result<Tensor> {
        println!("\n============================================================");
        println!("STEP 2: EMBEDDINGS (RUST)");
        println!("============================================================");
        
        println!("Testing embeddings with token: {}", token);
        
        let input_tensor = Tensor::from_vec(vec![token], (1, 1), &self.device)?;
        println!("Input shape: {:?}", input_tensor.shape());
        println!("Input tensor data: {:?}", input_tensor.to_vec2::<i64>()?);
        println!("Input tensor dtype: {:?}", input_tensor.dtype());
        
        // Try a simple token like 0 or 1 to see if that works
        println!("Testing with simple token 1...");
        let simple_tensor = Tensor::from_vec(vec![1i64], (1, 1), &self.device)?;
        let simple_hidden = self.embeddings.forward(&[&simple_tensor])?;
        let simple_vec = simple_hidden.to_vec3::<f32>()?;
        let simple_flat: Vec<f32> = simple_vec[0][0].clone();
        let simple_mean = simple_flat.iter().sum::<f32>() / simple_flat.len() as f32;
        println!("Simple token 1 output mean: {:.6}", simple_mean);
        
        // Also try F32 tensor like working tests use
        println!("Testing with F32 tensor [1.0]...");
        let f32_tensor = Tensor::from_vec(vec![1.0f32], (1, 1), &self.device)?;
        let f32_hidden = self.embeddings.forward(&[&f32_tensor])?;
        let f32_vec = f32_hidden.to_vec3::<f32>()?;
        let f32_flat: Vec<f32> = f32_vec[0][0].clone();
        let f32_mean = f32_flat.iter().sum::<f32>() / f32_flat.len() as f32;
        println!("F32 tensor [1.0] output mean: {:.6}", f32_mean);
        
        let hidden_states = self.embeddings.forward(&[&input_tensor])?;
        
        println!("Output shape: {:?}", hidden_states.shape());
        
        // Get stats
        let hidden_vec = hidden_states.to_vec3::<f32>()?;
        let hidden_flat: Vec<f32> = hidden_vec[0][0].clone();
        
        let min_val = hidden_flat.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let max_val = hidden_flat.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let mean_val = hidden_flat.iter().sum::<f32>() / hidden_flat.len() as f32;
        let variance = hidden_flat.iter().map(|&x| (x - mean_val).powi(2)).sum::<f32>() / hidden_flat.len() as f32;
        let std_val = variance.sqrt();
        
        println!("Output stats: min={:.6}, max={:.6}, mean={:.6}, std={:.6}", min_val, max_val, mean_val, std_val);
        println!("First 10 values: {:?}", &hidden_flat[..10]);
        
        Ok(hidden_states)
    }
    
    pub fn test_ffn(&self, hidden_states: &Tensor, position: usize) -> Result<Tensor> {
        println!("\n============================================================");
        println!("STEP 3: FFN (RUST) (position {})", position);
        println!("============================================================");
        
        // Create state
        let mut state = self.ffn.make_state()?;
        println!("Created MLState");
        
        // Create position tensors
        let position_ids = Tensor::from_vec(vec![position as i64], (1,), &self.device)?;
        let current_pos = Tensor::from_vec(vec![position as i64], (1,), &self.device)?;
        
        // Create causal mask - only allow access to positions up to current
        let mut mask_data = vec![f32::NEG_INFINITY; 512];
        for i in 0..=position {
            mask_data[i] = 0.0;
        }
        let causal_mask = Tensor::from_vec(mask_data, (1, 1, 1, 512), &self.device)?;
        
        println!("position_ids: {:?}", position_ids.to_vec1::<i64>()?);
        println!("current_pos: {:?}", current_pos.to_vec1::<i64>()?);
        println!("causal_mask shape: {:?}", causal_mask.shape());
        
        // Check first 5 mask values - need to extract properly from 4D tensor
        let mask_slice = causal_mask.i((0, 0, 0, 0..5))?;
        let mask_vec = mask_slice.to_vec1::<f32>()?;
        println!("causal_mask[0,0,0,:5]: {:?}", mask_vec);
        
        // Get input stats
        let input_vec = hidden_states.to_vec3::<f32>()?;
        let input_flat: Vec<f32> = input_vec[0][0].clone();
        let input_min = input_flat.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let input_max = input_flat.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let input_mean = input_flat.iter().sum::<f32>() / input_flat.len() as f32;
        
        println!("Input hidden shape: {:?}", hidden_states.shape());
        println!("Input stats: min={:.6}, max={:.6}, mean={:.6}", input_min, input_max, input_mean);
        
        // Run FFN
        let output_hidden = self.ffn.predict_with_state(&[
            hidden_states,
            &position_ids,
            &current_pos,
            &causal_mask
        ], &mut state)?;
        
        println!("Output hidden shape: {:?}", output_hidden.shape());
        
        // Get output stats
        let output_vec = output_hidden.to_vec3::<f32>()?;
        let output_flat: Vec<f32> = output_vec[0][0].clone();
        let output_min = output_flat.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let output_max = output_flat.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let output_mean = output_flat.iter().sum::<f32>() / output_flat.len() as f32;
        
        println!("Output stats: min={:.6}, max={:.6}, mean={:.6}", output_min, output_max, output_mean);
        println!("Input first 10 values: {:?}", &input_flat[..10]);
        println!("Output first 10 values: {:?}", &output_flat[..10]);
        
        Ok(output_hidden)
    }
    
    pub fn test_lm_head(&self, hidden_states: &Tensor) -> Result<Vec<f32>> {
        println!("\n============================================================");
        println!("STEP 4: LM HEAD (RUST)");
        println!("============================================================");
        
        // Get input stats
        let input_vec = hidden_states.to_vec3::<f32>()?;
        let input_flat: Vec<f32> = input_vec[0][0].clone();
        let input_min = input_flat.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let input_max = input_flat.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let input_mean = input_flat.iter().sum::<f32>() / input_flat.len() as f32;
        
        println!("Input hidden shape: {:?}", hidden_states.shape());
        println!("Input stats: min={:.6}, max={:.6}, mean={:.6}", input_min, input_max, input_mean);
        
        // Get all outputs from LM head
        let all_outputs = self.lm_head.forward_all(&[hidden_states])?;
        
        println!("LM head outputs: {:?}", all_outputs.keys().collect::<Vec<_>>());
        
        // Concatenate all logits chunks
        let mut full_logits = Vec::new();
        
        for i in 1..=16 {
            let key = format!("logits{}", i);
            if let Some(chunk_tensor) = all_outputs.get(&key) {
                let chunk_vec = chunk_tensor.to_vec3::<f32>()?;
                let chunk_flat = &chunk_vec[0][0];
                
                let chunk_min = chunk_flat.iter().fold(f32::INFINITY, |a, &b| a.min(b));
                let chunk_max = chunk_flat.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
                
                println!("  {}: shape {:?}, range [{:.3}, {:.3}]", key, chunk_tensor.shape(), chunk_min, chunk_max);
                
                full_logits.extend_from_slice(chunk_flat);
            }
        }
        
        println!("\nConcatenated logits: {} total", full_logits.len());
        
        let logits_min = full_logits.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let logits_max = full_logits.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let logits_mean = full_logits.iter().sum::<f32>() / full_logits.len() as f32;
        
        println!("Logits stats: min={:.6}, max={:.6}, mean={:.6}", logits_min, logits_max, logits_mean);
        
        // Find top 5 tokens
        let mut indexed_logits: Vec<(usize, f32)> = full_logits.iter().enumerate().map(|(i, &v)| (i, v)).collect();
        indexed_logits.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        println!("\nTop 5 tokens:");
        for (i, (token_id, score)) in indexed_logits.iter().take(5).enumerate() {
            println!("  {}. Token {}: {:.6}", i + 1, token_id, score);
        }
        
        Ok(full_logits)
    }
    
    pub fn test_sampling(&self, logits: &[f32], temperature: f32) -> Result<usize> {
        println!("\n============================================================");
        println!("STEP 5: SAMPLING (RUST) (temperature={})", temperature);
        println!("============================================================");
        
        if temperature == 0.0 {
            // Greedy sampling
            let (best_idx, best_score) = logits.iter().enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap();
            println!("Greedy sampling: token {}, score {:.6}", best_idx, best_score);
            Ok(best_idx)
        } else {
            // Temperature sampling
            let scaled_logits: Vec<f32> = logits.iter().map(|&x| x / temperature).collect();
            let max_logit = scaled_logits.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
            let exp_logits: Vec<f32> = scaled_logits.iter().map(|&x| (x - max_logit).exp()).collect();
            let sum: f32 = exp_logits.iter().sum();
            let probabilities: Vec<f32> = exp_logits.iter().map(|&x| x / sum).collect();
            
            println!("Temperature scaling applied");
            let prob_min = probabilities.iter().fold(f32::INFINITY, |a, &b| a.min(b));
            let prob_max = probabilities.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
            let prob_sum = probabilities.iter().sum::<f32>();
            println!("Probability stats: min={:.8}, max={:.8}, sum={:.8}", prob_min, prob_max, prob_sum);
            
            // Use fixed seed for reproducibility - simulate the Python random.seed(42)
            use rand::{Rng, SeedableRng};
            let mut rng = rand::rngs::StdRng::seed_from_u64(42);
            let random_value: f32 = rng.gen();
            
            let mut cumsum = 0.0;
            for (idx, &prob) in probabilities.iter().enumerate() {
                cumsum += prob;
                if random_value <= cumsum {
                    println!("Sampled token: {}, probability {:.8}", idx, prob);
                    return Ok(idx);
                }
            }
            
            // Fallback
            Ok(probabilities.len() - 1)
        }
    }
}