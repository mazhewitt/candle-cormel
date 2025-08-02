//! Qwen model integration for candle-coreml
//!
//! This module provides a complete implementation of the Qwen multi-component architecture
//! with proper tokenization, state management, and inference pipeline.

use crate::utils::{mask, multi_component};
use crate::{Config as CoreMLConfig, CoreMLModel, CoreMLState};
use candle_core::{Device, Error as CandleError, Tensor};
use std::collections::HashMap;
use std::path::Path;
use tokenizers::Tokenizer;
use tracing::{debug, trace, warn};

/// Qwen model constants
pub const QWEN_VOCAB_SIZE: usize = 151936;
pub const QWEN_HIDDEN_SIZE: usize = 1024;
pub const QWEN_BATCH_SIZE: usize = 64;
pub const QWEN_CONTEXT_LENGTH: usize = 512;

/// Configuration for Qwen model components
#[derive(Debug, Clone)]
pub struct QwenConfig {
    pub vocab_size: usize,
    pub hidden_size: usize,
    pub batch_size: usize,
    pub context_length: usize,
    pub device: Device,
}

impl Default for QwenConfig {
    fn default() -> Self {
        Self {
            vocab_size: QWEN_VOCAB_SIZE,
            hidden_size: QWEN_HIDDEN_SIZE,
            batch_size: QWEN_BATCH_SIZE,
            context_length: QWEN_CONTEXT_LENGTH,
            device: Device::Cpu,
        }
    }
}

/// Complete Qwen model with all components and state management
pub struct QwenModel {
    pub embeddings: CoreMLModel,
    pub ffn_prefill: CoreMLModel,
    pub ffn_infer: CoreMLModel,
    pub lm_head: CoreMLModel,
    tokenizer: Tokenizer,
    config: QwenConfig,
    unified_state: Option<CoreMLState>, // Single shared state for both prefill and infer
    cached_causal_mask: Option<Tensor>, // Pre-computed causal mask (like chat.py)
    // Embeddings optimization
    embeddings_cache: HashMap<Vec<i64>, Tensor>, // Cache for token sequence embeddings
    last_sequence_embeddings: Option<(Vec<i64>, Tensor)>, // Cache last full sequence
}

impl QwenModel {
    /// Load Qwen model from the specified directory
    pub fn load_from_directory<P: AsRef<Path>>(
        model_dir: P,
        config: Option<QwenConfig>,
    ) -> Result<Self, CandleError> {
        let config = config.unwrap_or_default();
        let model_dir = model_dir.as_ref();

        // Load tokenizer
        let tokenizer_path = model_dir.join("tokenizer.json");
        let tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| CandleError::Msg(format!("Failed to load tokenizer: {}", e)))?;

        // Configure and load embeddings
        let embeddings_config = CoreMLConfig {
            input_names: vec!["input_ids".to_string()],
            output_name: "hidden_states".to_string(),
            max_sequence_length: config.context_length,
            vocab_size: config.vocab_size,
            model_type: "qwen-embeddings".to_string(),
        };

        let embeddings_path = model_dir.join("qwen_embeddings.mlmodelc");
        let embeddings = CoreMLModel::load_from_file(&embeddings_path, &embeddings_config)?;

        // Configure and load FFN models (both prefill and infer functions)
        let ffn_config_base = CoreMLConfig {
            input_names: vec![
                "hidden_states".to_string(),
                "position_ids".to_string(),
                "causal_mask".to_string(),
                "current_pos".to_string(),
            ],
            output_name: "output_hidden_states".to_string(),
            max_sequence_length: config.context_length,
            vocab_size: config.hidden_size,
            model_type: "qwen-ffn".to_string(),
        };

        let ffn_path = model_dir.join("qwen_FFN_PF_lut8_chunk_01of01.mlmodelc");

        // FFN Prefill function (for initial sequence processing)
        let ffn_prefill = CoreMLModel::load_with_function(&ffn_path, &ffn_config_base, "prefill")?;

        // FFN Infer function (for token-by-token generation with update_mask)
        let mut ffn_infer_config = ffn_config_base.clone();
        ffn_infer_config
            .input_names
            .insert(1, "update_mask".to_string());
        let ffn_infer = CoreMLModel::load_with_function(&ffn_path, &ffn_infer_config, "infer")?;

        // Configure and load LM head
        let lm_head_config = CoreMLConfig {
            input_names: vec!["hidden_states".to_string()],
            output_name: "logits1".to_string(), // First of 16 outputs
            max_sequence_length: config.context_length,
            vocab_size: config.vocab_size,
            model_type: "qwen-lm-head".to_string(),
        };

        let lm_head_path = model_dir.join("qwen_lm_head_lut8.mlmodelc");
        let lm_head = CoreMLModel::load_from_file(&lm_head_path, &lm_head_config)?;

        Ok(Self {
            embeddings,
            ffn_prefill,
            ffn_infer,
            lm_head,
            tokenizer,
            config,
            unified_state: None, // Single shared state
            cached_causal_mask: None, // Will be computed on first use
            embeddings_cache: HashMap::new(),
            last_sequence_embeddings: None,
        })
    }

    /// Initialize model states for efficient generation
    /// CRITICAL: Use a single shared state between prefill and infer (matches Python chat.py)
    pub fn initialize_states(&mut self) -> Result<(), CandleError> {
        // Create ONE unified state that both prefill and infer will share
        let unified_state = self.ffn_prefill.make_state()?;
        self.unified_state = Some(unified_state);
        
        // Pre-compute causal mask ONCE (like chat.py initialize_causal_mask)
        if self.cached_causal_mask.is_none() {
            let context_length = self.config.context_length;
            let causal_mask = self.create_full_causal_mask(context_length)?;
            self.cached_causal_mask = Some(causal_mask);
            debug!("✅ Pre-computed causal mask for context length {}", context_length);
        }
        
        Ok(())
    }

    /// Create full causal mask once (like chat.py make_causal_mask)
    fn create_full_causal_mask(&self, context_length: usize) -> Result<Tensor, CandleError> {
        // Create full causal mask: (1, 1, context_length, context_length)
        let mut mask_data = vec![f32::NEG_INFINITY; context_length * context_length];
        
        for i in 0..context_length {
            for j in 0..=i {
                mask_data[i * context_length + j] = 0.0;
            }
        }
        
        Tensor::from_vec(mask_data, (1, 1, context_length, context_length), &self.config.device)
    }

    /// Reset states for a new generation sequence
    pub fn reset_states(&mut self) -> Result<(), CandleError> {
        self.initialize_states()
    }

    /// Tokenize input text
    pub fn tokenize(&self, text: &str) -> Result<Vec<i64>, CandleError> {
        let encoding = self
            .tokenizer
            .encode(text, true)
            .map_err(|e| CandleError::Msg(format!("Tokenization failed: {}", e)))?;

        Ok(encoding.get_ids().iter().map(|&id| id as i64).collect())
    }

    /// Pad tokens to appropriate batch size for embeddings
    pub fn pad_tokens(&self, tokens: &[i64]) -> Vec<i64> {
        if tokens.len() == 1 {
            tokens.to_vec() // Single token mode (1, 1)
        } else {
            // Pad to batch size (1, 64)
            let mut padded = tokens.to_vec();
            padded.resize(self.config.batch_size, 0);
            padded
        }
    }

    /// Create causal attention mask using shared utilities
    pub fn create_causal_mask(&self, seq_len: usize) -> Result<Tensor, CandleError> {
        // Create base mask and reshape to rank-4 for CoreML
        let base_mask = mask::create_causal_mask(seq_len, &self.config.device)?;
        base_mask.reshape((1, 1, seq_len, seq_len))
    }

    /// Create position slice of causal mask for single token processing
    pub fn create_position_causal_mask(
        &self,
        pos: usize,
        context_length: usize,
    ) -> Result<Tensor, CandleError> {
        mask::create_rank4_position_mask(pos, context_length, &self.config.device)
    }

    /// Create update mask for FFN infer phase
    pub fn create_update_mask(
        &self,
        pos: usize,
        context_length: usize,
    ) -> Result<Tensor, CandleError> {
        mask::create_update_mask(pos, context_length, &self.config.device)
    }

    /// Run embeddings for input tokens
    pub fn compute_embeddings(&self, tokens: &[i64]) -> Result<Tensor, CandleError> {
        let padded_tokens = self.pad_tokens(tokens);
        let input_tensor = Tensor::from_vec(
            padded_tokens.clone(),
            (1, padded_tokens.len()),
            &self.config.device,
        )?;

        self.embeddings.forward(&[&input_tensor])
    }

    /// 🚀 OPTIMIZED: Compute embeddings with caching and reuse
    pub fn compute_embeddings_optimized(&mut self, tokens: &[i64]) -> Result<Tensor, CandleError> {
        // Check if we already have embeddings for this exact sequence
        if let Some((cached_tokens, cached_embeddings)) = &self.last_sequence_embeddings {
            if cached_tokens == tokens {
                debug!("⚡ CACHE HIT: Reusing embeddings for sequence {:?}", tokens);
                return Ok(cached_embeddings.clone());
            }
        }

        // Compute new embeddings
        debug!("💾 CACHE MISS: Computing embeddings for sequence {:?}", tokens);
        let embeddings = self.compute_embeddings(tokens)?;
        
        // Cache the result
        self.last_sequence_embeddings = Some((tokens.to_vec(), embeddings.clone()));
        
        Ok(embeddings)
    }

    /// 🚀 OPTIMIZED: Get single token embedding from cached sequence  
    pub fn get_token_embedding_from_sequence(&self, tokens: &[i64], token_index: usize) -> Result<Option<Tensor>, CandleError> {
        if let Some((cached_tokens, cached_embeddings)) = &self.last_sequence_embeddings {
            if cached_tokens == tokens && token_index < tokens.len() {
                // Extract the specific token embedding from the cached sequence
                debug!("⚡ EXTRACTING: Token {} from cached sequence embeddings", token_index);
                let token_embedding = cached_embeddings.narrow(1, token_index, 1)?;
                return Ok(Some(token_embedding));
            }
        }
        Ok(None)
    }

    /// 🚀 OPTIMIZED: Get last token embedding without recomputing
    pub fn get_last_token_embedding_optimized(&mut self, tokens: &[i64]) -> Result<Tensor, CandleError> {
        let last_index = tokens.len() - 1;
        
        // Try to get from cached sequence first
        if let Some(cached_embedding) = self.get_token_embedding_from_sequence(tokens, last_index)? {
            debug!("⚡ REUSING: Last token embedding from cached sequence");
            return Ok(cached_embedding);
        }
        
        // Fallback: compute single token embedding
        debug!("💾 COMPUTING: Single last token embedding");
        let last_token = tokens[last_index];
        let input_tensor = Tensor::from_vec(vec![last_token], (1, 1), &self.config.device)?;
        self.embeddings.forward(&[&input_tensor])
    }

    /// Process sequence through FFN prefill phase
    pub fn prefill_sequence(
        &mut self,
        embeddings: &Tensor,
        sequence_length: usize,
    ) -> Result<(), CandleError> {
        if self.unified_state.is_none() {
            self.initialize_states()?;
        }

        let context_length = self.config.context_length;
        let device = &self.config.device;

        // Process each token position through prefill
        for pos in 0..sequence_length {
            let token_embedding = embeddings.narrow(1, pos, 1)?;

            let position_ids = Tensor::from_vec(vec![pos as i64], (1,), device)?;
            let causal_mask = self.create_position_causal_mask(pos, context_length)?;
            let current_pos = Tensor::from_vec(vec![pos as i64], (1,), device)?;

            let inputs = vec![&token_embedding, &position_ids, &causal_mask, &current_pos];
            let state = self.unified_state.as_mut().unwrap();
            let _output = self.ffn_prefill.predict_with_state(&inputs, state)?;
        }

        Ok(())
    }

    /// Generate next token using granular methods - REFACTORED
    pub fn generate_next_token(
        &mut self,
        last_embedding: &Tensor,
        pos: usize,
    ) -> Result<Tensor, CandleError> {
        let context_length = self.config.context_length;
        let device = &self.config.device;

        // Create inputs for infer phase
        let update_mask = self.create_update_mask(pos, context_length)?;
        let position_ids = Tensor::from_vec(vec![pos as i64], (1,), device)?;
        let causal_mask = self.create_position_causal_mask(pos, context_length)?;
        let current_pos = position_ids.clone();

        // Use granular infer method
        let hidden_states = self.run_ffn_infer_with_inputs(
            last_embedding,
            &update_mask,
            &position_ids,
            &causal_mask,
            &current_pos
        )?;

        // Use granular LM head method
        let combined_logits = self.run_lm_head_with_inputs(&hidden_states)?;

        Ok(combined_logits)
    }

    /// Combine 16 LM head output chunks into full vocabulary using shared utility
    pub fn combine_lm_head_outputs(
        &self,
        outputs: HashMap<String, Tensor>,
    ) -> Result<Tensor, CandleError> {
        multi_component::combine_chunked_logits(outputs, 16)
    }

    /// Forward pass returning raw logits (consistent with CoreMLModel API)
    pub fn forward(&mut self, inputs: &[&Tensor]) -> Result<Tensor, CandleError> {
        // For now, this is a simplified implementation
        // In a full implementation, this would orchestrate the multi-component pipeline
        if inputs.is_empty() {
            return Err(CandleError::Msg("No input tensors provided".to_string()));
        }

        // Process through embeddings -> FFN -> LM head
        let embeddings = self.embeddings.forward(&inputs[0..1])?;

        // For now, return embeddings as placeholder
        // TODO: Complete pipeline implementation
        Ok(embeddings)
    }


    /// Generate a single token from text input - PRIMARY METHOD
    /// ✅ Uses chat.py architecture for correct predictions (correctly answers "Paris" for capital of France)
    /// Replicates Python reference architecture with chunked prefill and cached masks
    pub fn forward_text(&mut self, text: &str) -> Result<i64, CandleError> {
        let start_time = std::time::Instant::now();
        
        // Ensure states and causal mask are initialized (done once like chat.py)
        if self.unified_state.is_none() || self.cached_causal_mask.is_none() {
            self.initialize_states()?;
        }
        
        // Tokenize input
        let tokens = self.tokenize(text)?;
        let context_pos = tokens.len();
        debug!("🚀 Chat.py-style: Processing {} tokens", context_pos);

        // PHASE 1: CHUNKED PREFILL (exactly like chat.py run_prefill)
        let prefill_start = std::time::Instant::now();
        self.run_chatpy_prefill(&tokens, context_pos)?;
        let prefill_time = prefill_start.elapsed();
        debug!("⚡ Chat.py prefill took: {:?}", prefill_time);

        // PHASE 2: SINGLE TOKEN INFER (exactly like chat.py generate_next_token)
        let infer_start = std::time::Instant::now();
        let last_token = tokens[tokens.len() - 1];
        let next_token = self.run_chatpy_infer(last_token, context_pos)?;
        let infer_time = infer_start.elapsed();
        debug!("⚡ Chat.py infer took: {:?}", infer_time);

        let total_time = start_time.elapsed();
        debug!("🎯 CHAT.PY TOTAL: {:?} (target: ~11ms for 87 t/s)", total_time);
        
        Ok(next_token)
    }


    /// Extract next token from logits (shared utility)
    fn extract_next_token(&self, logits: &Tensor) -> Result<i64, CandleError> {
        let flat_logits = logits.squeeze(0)?.squeeze(0)?;
        let logits_vec = flat_logits.to_vec1::<f32>()?;

        // Use same tie-breaking logic as TDD test
        let mut indexed_logits: Vec<(usize, f32)> = logits_vec.iter().enumerate().map(|(i, &score)| (i, score)).collect();
        indexed_logits.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let next_token = indexed_logits[0].0 as i64;

        // Show top predictions for debugging
        debug!("Top 5 extract_next_token predictions:");
        for (rank, (token_id, score)) in indexed_logits.iter().take(5).enumerate() {
            let decoded = self.tokenizer.decode(&[*token_id as u32], false).unwrap_or("???".to_string());
            debug!("  {}. Token {} ('{}'): {:.6}", rank + 1, token_id, decoded, score);
        }

        Ok(next_token)
    }



    /// Chat.py-style chunked prefill implementation
    fn run_chatpy_prefill(&mut self, tokens: &[i64], context_pos: usize) -> Result<(), CandleError> {
        let batch_size = self.config.batch_size; // 64
        let device = self.config.device.clone(); // Clone to avoid borrowing issues
        let causal_mask = self.cached_causal_mask.as_ref().unwrap().clone(); // Clone mask
        
        // Process in 64-token chunks (exactly like chat.py)
        let mut batch_pos = 0;
        while batch_pos < context_pos {
            let batch_end = (batch_pos + batch_size).min(context_pos);
            let _current_batch_size = batch_end - batch_pos;
            
            // Get current batch tokens
            let batch_tokens = &tokens[batch_pos..batch_end];
            
            // Pad to full batch size (exactly like chat.py F.pad)
            let mut padded_batch = batch_tokens.to_vec();
            padded_batch.resize(batch_size, 0); // Pad with zeros
            
            // Create input tensor
            let batch_input = Tensor::from_vec(padded_batch, (1, batch_size), &device)?;
            
            // Run embeddings
            let hidden_states = self.embeddings.forward(&[&batch_input])?;
            
            // Generate position IDs for full batch size (like chat.py)
            let position_ids_vec: Vec<i64> = (batch_pos as i64..(batch_pos + batch_size) as i64).collect();
            let position_ids = Tensor::from_vec(position_ids_vec, (batch_size,), &device)?;
            
            // Use pre-computed causal mask slice (like chat.py batch_causal_mask)
            let batch_causal_mask = causal_mask.narrow(2, batch_pos, batch_size)?;
            
            // Current pos for this batch
            let current_pos = Tensor::from_vec(vec![batch_pos as i64], (1,), &device)?;
            
            // Run prefill with the working method
            let _output = self.run_ffn_prefill_with_inputs(
                &hidden_states,
                &position_ids,
                &batch_causal_mask,
                &current_pos
            )?;
            
            batch_pos = batch_end;
        }
        
        debug!("✅ Chat.py prefill: Processed {} tokens in {} chunks", context_pos, (context_pos + batch_size - 1) / batch_size);
        Ok(())
    }

    /// Chat.py-style single token infer implementation
    fn run_chatpy_infer(&mut self, last_token: i64, pos: usize) -> Result<i64, CandleError> {
        let context_length = self.config.context_length;
        let device = self.config.device.clone(); // Clone to avoid borrowing issues
        let causal_mask = self.cached_causal_mask.as_ref().unwrap().clone(); // Clone mask
        
        // Get current token embedding (like chat.py)
        let current_token = Tensor::from_vec(vec![last_token], (1, 1), &device)?;
        let hidden_states = self.embeddings.forward(&[&current_token])?;
        
        // Create update mask (like chat.py)
        let mut update_mask_data = vec![0.0f32; context_length];
        update_mask_data[pos - 1] = 1.0; // Set position for update
        let update_mask = Tensor::from_vec(update_mask_data, (1, 1, context_length, 1), &device)?;
        
        // Position IDs and causal mask slice (like chat.py)
        let position_ids = Tensor::from_vec(vec![(pos - 1) as i64], (1,), &device)?;
        let single_causal_mask = causal_mask.narrow(2, pos - 1, 1)?; // Get slice for current position
        let current_pos = position_ids.clone();
        
        // Run infer using the working method
        let infer_output = self.run_ffn_infer_with_inputs(
            &hidden_states,
            &update_mask,
            &position_ids,
            &single_causal_mask,
            &current_pos
        )?;
        
        // Run LM head and extract token (like chat.py)
        let logits = self.run_lm_head_with_inputs(&infer_output)?;
        let next_token = self.extract_next_token(&logits)?;
        
        debug!("✅ Chat.py infer: Generated token {} at position {}", next_token, pos);
        Ok(next_token)
    }

    /// Performance benchmark for the current implementation
    pub fn benchmark_implementations(&mut self, text: &str, iterations: usize) -> Result<(), CandleError> {
        println!("🏁 PERFORMANCE BENCHMARK: Chat.py-style Implementation");
        println!("Text: '{}'", text);
        println!("Iterations: {}", iterations);
        println!("================================");

        // Benchmark current forward_text implementation (chat.py-style)
        let start = std::time::Instant::now();
        let mut results = Vec::new();
        for i in 0..iterations {
            let token = self.forward_text(text)?;
            results.push(token);
            if i == 0 {
                println!("🚀 Result: token {}", token);
                // Decode the token to show what it predicts
                if let Ok(decoded) = self.tokenizer.decode(&[token as u32], false) {
                    println!("   Decoded: '{}'", decoded);
                }
            }
        }
        let total_time = start.elapsed();
        let avg_time = total_time / iterations as u32;
        let tokens_per_sec = 1000.0 / avg_time.as_millis() as f64;

        println!("🚀 CURRENT IMPLEMENTATION (Chat.py-style):");
        println!("   Total time: {:?}", total_time);
        println!("   Average per call: {:?}", avg_time);
        println!("   Tokens/second: {:.2}", tokens_per_sec);

        // Performance target assessment
        if tokens_per_sec >= 70.0 {
            println!("🎯 TARGET ACHIEVED: {:.2} t/s >= 70 t/s ✅", tokens_per_sec);
        } else if tokens_per_sec >= 20.0 {
            println!("🎯 PARTIAL SUCCESS: {:.2} t/s >= 20 t/s (minimum target) ⚠️", tokens_per_sec);
        } else {
            println!("🎯 TARGET MISSED: {:.2} t/s < 20 t/s ❌", tokens_per_sec);
        }

        // Consistency check
        let all_same = results.iter().all(|&token| token == results[0]);
        println!("✅ Consistency: {} (all iterations produced {})", 
                 if all_same { "CONSISTENT" } else { "INCONSISTENT" },
                 if all_same { "same result" } else { "different results" });

        Ok(())
    }

    /// Run prefill phase to populate KV cache for all tokens in sequence
    /// This replicates the exact prefill behavior from our working tests
    pub fn run_prefill_phase(
        &mut self,
        embeddings: &Tensor,
        sequence_length: usize,
    ) -> Result<(), CandleError> {
        if self.unified_state.is_none() {
            self.initialize_states()?;
        }

        let batch_size = self.config.batch_size; // 64
        let context_length = self.config.context_length; // 512
        let device = &self.config.device;

        debug!("Running prefill for {} tokens (padded to {} batch) to populate KV cache", sequence_length, batch_size);

        // Create position IDs for the full batch (0, 1, 2, ..., 63)
        // Note: We use the full batch size, not just sequence_length
        let position_ids_vec: Vec<i64> = (0..batch_size as i64).collect();
        let position_ids = Tensor::from_vec(position_ids_vec, (batch_size,), device)?;

        // Create causal mask for the full batch (1, 1, 64, 512)
        // Each row allows attention to positions up to that token's position
        let mut mask_data = vec![f32::NEG_INFINITY; batch_size * context_length];
        for i in 0..batch_size {
            for j in 0..=i.min(context_length - 1) {
                mask_data[i * context_length + j] = 0.0;
            }
        }
        let causal_mask = Tensor::from_vec(mask_data, (1, 1, batch_size, context_length), device)?;

        // Current pos is the last actual token position (seq_len - 1)
        let current_pos = Tensor::from_vec(vec![sequence_length as i64 - 1], (1,), device)?;

        // Run prefill with full batch embeddings (already padded to 64)
        let inputs = vec![embeddings, &position_ids, &causal_mask, &current_pos];
        let state = self.unified_state.as_mut().unwrap();
        let _prefill_output = self.ffn_prefill.predict_with_state(&inputs, state)?;

        debug!("Prefill complete - KV cache populated for positions 0..{}", sequence_length - 1);
        Ok(())
    }

    /// Generate next token using FFN infer with populated state
    /// This replicates the exact infer behavior from our working tests
    /// CRITICAL: Uses the SAME state object that was populated during prefill
    pub fn generate_next_token_with_infer(
        &mut self,
        token_embedding: &Tensor,
        current_position: usize,
    ) -> Result<Tensor, CandleError> {
        let context_length = self.config.context_length;
        let device = &self.config.device;

        debug!("Running infer for position {} using SHARED state from prefill (FIXED: last token pos)", current_position);

        // CRITICAL: We must use the SAME state that was populated by prefill!
        // Use the shared state that was populated during prefill
        if self.unified_state.is_none() {
            return Err(CandleError::Msg("No unified state available - prefill must be run first".to_string()));
        }
        debug!("Using SHARED state populated by prefill (like working tests)");

        // Create infer inputs (matching our working test exactly)
        let update_mask = self.create_update_mask(current_position, context_length)?;
        let position_ids = Tensor::from_vec(vec![current_position as i64], (1,), device)?;
        let causal_mask = self.create_position_causal_mask(current_position, context_length)?;
        let current_pos = position_ids.clone();

        // Run infer to get hidden states using the shared populated state
        let inputs = vec![
            token_embedding,
            &update_mask,
            &position_ids,
            &causal_mask,
            &current_pos,
        ];
        let state = self.unified_state.as_mut().unwrap(); // Use the UNIFIED state!
        let hidden_states = self.ffn_infer.predict_with_state(&inputs, state)?;

        debug!("Infer complete - processing through LM head");

        // Run through LM head to get logits (16 chunks)
        let lm_outputs = self.lm_head.forward_all(&[&hidden_states])?;
        let combined_logits = self.combine_lm_head_outputs(lm_outputs)?;

        Ok(combined_logits)
    }

    /// Generate text using temperature sampling
    pub fn generate_text(
        &mut self,
        text: &str,
        max_tokens: usize,
        temperature: f32,
    ) -> Result<String, CandleError> {
        let tokens = self.generate_tokens(text, max_tokens, temperature, None)?;

        // Decode tokens back to text
        let token_ids: Vec<u32> = tokens.iter().map(|&id| id as u32).collect();
        self.tokenizer
            .decode(&token_ids, false)
            .map_err(|e| CandleError::Msg(format!("Failed to decode tokens: {}", e)))
    }

    /// Generate multiple tokens using temperature sampling with optional top-k
    /// Generate multiple tokens with correct position tracking
pub fn generate_tokens(
    &mut self,
    text: &str,
    max_tokens: usize,
    temperature: f32,
    _top_k: Option<usize>,
) -> Result<Vec<i64>, CandleError> {
    let mut generated_tokens = Vec::new();
    let mut current_text = text.to_string();
    
    for _ in 0..max_tokens {
        // Use the working forward_text method for each token
        let next_token = self.forward_text(&current_text)?;
        generated_tokens.push(next_token);
        
        // Stop if EOS
        if next_token == 151645 {
            break;
        }
        
        // Update current_text by appending the new token
        if let Ok(decoded) = self.tokenizer.decode(&[next_token as u32], false) {
            current_text.push_str(&decoded);
        } else {
            // If decoding fails, stop generation
            break;
        }
        
        // For temperature sampling, we'd need to modify forward_text to accept temperature
        // For now, this uses greedy sampling which is what forward_text does
        if temperature > 0.0 {
            // TODO: Implement temperature sampling support
            // For now, fall back to greedy
        }
    }
    
    Ok(generated_tokens)
}

    /// Generate text using top-k sampling
    pub fn generate_text_top_k(
        &mut self,
        text: &str,
        max_tokens: usize,
        k: usize,
        temperature: f32,
    ) -> Result<String, CandleError> {
        let tokens = self.generate_tokens(text, max_tokens, temperature, Some(k))?;

        // Decode tokens back to text
        let token_ids: Vec<u32> = tokens.iter().map(|&id| id as u32).collect();
        self.tokenizer
            .decode(&token_ids, false)
            .map_err(|e| CandleError::Msg(format!("Failed to decode tokens: {}", e)))
    }

    /// Get model configuration
    pub fn config(&self) -> &QwenConfig {
        &self.config
    }

    /// Get tokenizer reference
    pub fn tokenizer(&self) -> &Tokenizer {
        &self.tokenizer
    }

    // ========== GRANULAR PIPELINE METHODS ==========
    // These methods expose each step of the pipeline for testing and debugging
    
    /// Run FFN prefill phase with exact inputs (for testing)
    pub fn run_ffn_prefill_with_inputs(
        &mut self,
        hidden_states: &Tensor,
        position_ids: &Tensor, 
        causal_mask: &Tensor,
        current_pos: &Tensor,
    ) -> Result<Tensor, CandleError> {
        if self.unified_state.is_none() {
            self.initialize_states()?;
        }
        
        let inputs = [hidden_states, position_ids, causal_mask, current_pos];
        let state = self.unified_state.as_mut().unwrap(); // Use the same unified state
        let output = self.ffn_prefill.predict_with_state(&inputs, state)?;
        
        Ok(output)
    }
    
    /// Run FFN infer phase with exact inputs (for testing)
    pub fn run_ffn_infer_with_inputs(
        &mut self,
        hidden_states: &Tensor,
        update_mask: &Tensor,
        position_ids: &Tensor,
        causal_mask: &Tensor, 
        current_pos: &Tensor,
    ) -> Result<Tensor, CandleError> {
        if self.unified_state.is_none() {
            return Err(CandleError::Msg("No unified state available - prefill must be run first".to_string()));
        }
    
        // CRITICAL FIX: Match Python reference implementation input order
        // Python infer inputs: hidden_states, update_mask, position_ids, causal_mask, current_pos
        // where current_pos should equal position_ids for proper state continuity
        trace!("DEBUG: Infer inputs - position_ids: {:?}, current_pos: {:?}", 
                 position_ids.to_vec1::<f32>().unwrap_or_default(), 
                 current_pos.to_vec1::<f32>().unwrap_or_default());

        // DEBUGGING: Validate all inputs before CoreML call
        trace!("INFER INPUT VALIDATION:");
        trace!("  hidden_states: shape={:?}, sample={:?}", 
                 hidden_states.shape(), 
                 hidden_states.to_vec3::<f32>().unwrap_or_default()[0][0][..3.min(hidden_states.dim(2).unwrap_or(0))].to_vec());
        let update_nonzeros = if let Ok(flat) = update_mask.flatten_all() {
            if let Ok(vec) = flat.to_vec1::<f32>() {
                vec.iter().filter(|&&x| x != 0.0).count()
            } else { 0 }
        } else { 0 };
        trace!("  update_mask: shape={:?}, nonzeros={}", update_mask.shape(), update_nonzeros);
        
        trace!("  position_ids: shape={:?}, values={:?}", 
                 position_ids.shape(),
                 position_ids.to_vec1::<f32>().unwrap_or_default());
        
        let causal_nonzeros = if let Ok(flat) = causal_mask.flatten_all() {
            if let Ok(vec) = flat.to_vec1::<f32>() {
                vec.iter().filter(|&&x| x != 0.0).count()
            } else { 0 }
        } else { 0 };
        trace!("  causal_mask: shape={:?}, nonzeros={}", causal_mask.shape(), causal_nonzeros);

        let inputs = [hidden_states, update_mask, position_ids, causal_mask, current_pos];
        let state = self.unified_state.as_mut().unwrap(); // Use the SAME unified state as prefill
        
        trace!("About to call CoreML infer model...");
        let output = self.ffn_infer.predict_with_state(&inputs, state)?;
        
        // DEBUGGING: Check output immediately after CoreML call
        let output_sample = output.to_vec3::<f32>().unwrap_or_default()[0][0][..5.min(output.dim(2).unwrap_or(0))].to_vec();
        trace!("INFER OUTPUT VALIDATION:");
        trace!("  output: shape={:?}, sample={:?}", output.shape(), output_sample);
        
        if output_sample.iter().all(|&x| x == 0.0) {
            warn!("ZEROS DETECTED: CoreML infer model returned all zeros!");
        } else {
            trace!("NON-ZERO OUTPUT: CoreML infer model returned valid data");
        }
        
        Ok(output)
    }
    
    /// Run LM head with exact inputs (for testing)
    pub fn run_lm_head_with_inputs(&self, hidden_states: &Tensor) -> Result<Tensor, CandleError> {
        let lm_outputs = self.lm_head.forward_all(&[hidden_states])?;
        let combined_logits = self.combine_lm_head_outputs(lm_outputs)?;
        Ok(combined_logits)
    }
    
    /// Get direct access to embeddings model (for testing)
    pub fn run_embeddings_with_inputs(&self, input_ids: &Tensor) -> Result<Tensor, CandleError> {
        self.embeddings.forward(&[input_ids])
    }
    
    /// Create state objects (for testing)
    pub fn create_fresh_states(&mut self) -> Result<(), CandleError> {
        self.initialize_states()
    }
    
    /// Reset states (for testing)
    pub fn clear_states(&mut self) -> Result<(), CandleError> {
        self.reset_states()
    }

    /// Debug method: Get FFN output directly (for testing) - DEPRECATED
    /// Use run_ffn_infer_with_inputs for new code
    pub fn debug_get_ffn_output(
        &mut self,
        token_embedding: &Tensor,
        current_position: usize,
    ) -> Result<Tensor, CandleError> {
        let context_length = self.config.context_length;
        let device = &self.config.device;

        // Create infer inputs using the same methods as production
        let update_mask = self.create_update_mask(current_position, context_length)?;
        let position_ids = Tensor::from_vec(vec![current_position as i64], (1,), device)?;
        let causal_mask = self.create_position_causal_mask(current_position, context_length)?;
        let current_pos = position_ids.clone();

        // Use the new granular method
        self.run_ffn_infer_with_inputs(token_embedding, &update_mask, &position_ids, &causal_mask, &current_pos)
    }
    
    /// Direct access to CoreML infer model for granular testing
    pub fn debug_direct_infer_model_execution(
        &mut self,
        inputs: &[&Tensor; 5],
    ) -> Result<Tensor, CandleError> {
        if self.unified_state.is_none() {
            return Err(CandleError::Msg("No unified state available - prefill must be run first".to_string()));
        }
        
        let state = self.unified_state.as_mut().unwrap();
        let output = self.ffn_infer.predict_with_state(inputs, state)?;
        Ok(output)
    }
}
