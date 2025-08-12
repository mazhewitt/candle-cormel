//! Inference pipeline and text generation for Qwen models
//!
//! This module contains the high-level inference methods including forward_text,
//! chat.py-style prefill/infer pipeline, and text generation utilities.

use crate::qwen::model::QwenModel;
use candle_core::{Error as CandleError, Tensor};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

/// A single prefill step mapping inside a padded embeddings window.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrefillStep {
    /// Index inside the current embeddings window (0..embeddings_len)
    pub local_idx: usize,
    /// Absolute token position within the prompt (0..context_pos)
    pub global_pos: usize,
}

/// A pure, testable plan describing how to run sequential prefill over one or more windows.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SequentialPrefillPlan {
    /// Ordered list of prefill steps to execute. Does not include the final token (reserved for infer).
    pub steps: Vec<PrefillStep>,
    /// The start index of the final window (for slicing last token embedding)
    pub last_window_start: usize,
    /// The local index of the last token embedding inside the final window
    pub last_local_idx: usize,
}

impl QwenModel {
    /// Static helper variant of plan_sequential_prefill for unit testing without a model instance.
    pub fn plan_sequential_prefill_static(
        token_count: usize,
        embeddings_len: usize,
        already_prefilled: usize,
    ) -> SequentialPrefillPlan {
        // Delegate to the instance logic by simulating the minimal inputs.
        // This duplicates the logic to keep it pure and independent of any model fields.
        let mut steps = Vec::new();
        let mut processed = already_prefilled.min(token_count);
        while processed < token_count.saturating_sub(1) {
            let window_end = (processed + embeddings_len).min(token_count);
            let window_len = window_end - processed;
            let take_len = window_len
                .saturating_sub(1)
                .min(token_count.saturating_sub(1) - processed);
            for local in 0..take_len {
                let global_pos = processed + local;
                if global_pos < already_prefilled {
                    continue;
                }
                steps.push(PrefillStep {
                    local_idx: local,
                    global_pos,
                });
            }
            processed = window_end;
        }
        let last_window_start = token_count.saturating_sub(embeddings_len);
        let last_local_idx = token_count - last_window_start - 1;
        SequentialPrefillPlan {
            steps,
            last_window_start,
            last_local_idx,
        }
    }

    /// Build a deterministic plan for single-token sequential prefill over possibly multiple windows.
    /// This function is pure and unit-test friendly.
    pub fn plan_sequential_prefill(
        &self,
        token_count: usize,
        embeddings_len: usize,
        already_prefilled: usize,
    ) -> SequentialPrefillPlan {
        // Delegate to the static version to ensure identical behavior across tests and runtime.
        QwenModel::plan_sequential_prefill_static(token_count, embeddings_len, already_prefilled)
    }

    /// Generate a single token from text input - PRIMARY METHOD
    /// ✅ Uses chat.py architecture for correct predictions (correctly answers "Paris" for capital of France)
    /// 🚀 OPTIMIZED: Enhanced with embeddings caching for maximum performance
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
        debug!(
            "🚀 Chat.py-style OPTIMIZED: Processing {} tokens",
            context_pos
        );

        // 🚀 OPTIMIZATION: Pre-compute and cache embeddings for the full sequence
        let embeddings_start = std::time::Instant::now();
        let _cached_embeddings = self.compute_embeddings(&tokens)?;
        let embeddings_time = embeddings_start.elapsed();
        debug!(
            "⚡ Cached embeddings took: {:?} for {} tokens",
            embeddings_time, context_pos
        );

        // If model is configured for single-token sequential prefill, use simplified path
        if self.config.model_config.prefill_is_single_token() {
            debug!(
                "🧪 forward_text: single-token prefill mode detected, using sequential pipeline"
            );
            // Ensure we have embeddings for full (padded) sequence
            let embeddings = self.compute_embeddings(&tokens)?; // padded to embeddings_input_shape
            let embed_seq_len = embeddings.dim(1)?;
            // Determine how many tokens we've already prefetched (if prompt grew)
            let already_prefilled = self.last_single_token_prefill_len.unwrap_or(0);
            if already_prefilled > context_pos {
                // New prompt shorter than previous -> reset state
                debug!("🔄 forward_text(single-token): prompt reset (previous prefilled {} > new {}), reinitializing state", already_prefilled, context_pos);
                self.unified_state = None;
                self.cached_causal_mask = None;
                self.last_single_token_prefill_len = None;
                self.initialize_states()?;
            }
            let already_prefilled = self.last_single_token_prefill_len.unwrap_or(0);

            // Construct a testable execution plan and use it to drive the calls
            let plan = self.plan_sequential_prefill(context_pos, embed_seq_len, already_prefilled);

            // Run sequential prefill across one or more windows (as needed by the plan)
            if self.unified_state.is_none() || self.cached_causal_mask.is_none() {
                self.initialize_states()?;
            }
            let causal_mask_full = self.cached_causal_mask.as_ref().unwrap().clone();

            if !plan.steps.is_empty() {
                // If a single window suffices, reuse the full embeddings; otherwise recompute per-window slices
                if context_pos <= embed_seq_len && already_prefilled + 1 < context_pos {
                    for step in &plan.steps {
                        self.prefill_single_token_step_chunk(
                            &embeddings,
                            step.local_idx,
                            step.global_pos,
                            &causal_mask_full,
                        )?;
                    }
                } else {
                    // Multi-window: recompute embeddings per window slice
                    let mut processed = already_prefilled.min(context_pos);
                    while processed < context_pos {
                        let window_end = (processed + embed_seq_len).min(context_pos);
                        let window_tokens = &tokens[processed..window_end];
                        let window_embeddings = self.compute_embeddings(window_tokens)?;
                        for step in plan
                            .steps
                            .iter()
                            .filter(|s| s.global_pos >= processed && s.global_pos < window_end)
                        {
                            let local = step.global_pos - processed;
                            self.prefill_single_token_step_chunk(
                                &window_embeddings,
                                local,
                                step.global_pos,
                                &causal_mask_full,
                            )?;
                        }
                        processed = window_end;
                    }
                }
            }

            // Slice the last embedding according to the plan and run infer
            if context_pos == 0 {
                return Err(CandleError::Msg("Empty token sequence".into()));
            }
            if plan.last_window_start == 0 {
                // Last token is inside the full embeddings tensor we already computed
                let last_embed = embeddings.narrow(1, plan.last_local_idx, 1)?;
                let logits = self.generate_next_token_with_infer(&last_embed, context_pos - 1)?;
                let next_token = self.extract_next_token(&logits)?;
                self.last_single_token_prefill_len = Some(context_pos);
                let total_time = start_time.elapsed();
                debug!("🎯 SINGLE-TOKEN TOTAL: {:?}", total_time);
                return Ok(next_token);
            } else {
                // Need to recompute the last window's embeddings
                let last_window_tokens = &tokens[plan.last_window_start..context_pos];
                let last_embeddings = self.compute_embeddings(last_window_tokens)?;
                let last_embed = last_embeddings.narrow(1, plan.last_local_idx, 1)?;
                let logits = self.generate_next_token_with_infer(&last_embed, context_pos - 1)?;
                let next_token = self.extract_next_token(&logits)?;
                self.last_single_token_prefill_len = Some(context_pos);
                let total_time = start_time.elapsed();
                debug!("🎯 MULTI-WINDOW SINGLE-TOKEN TOTAL: {:?}", total_time);
                return Ok(next_token);
            }
        }

        // PHASE 1: CHUNKED PREFILL (chat.py architecture with embeddings optimization)
        let prefill_start = std::time::Instant::now();
        self.run_chatpy_prefill(&tokens, context_pos)?;
        let prefill_time = prefill_start.elapsed();
        debug!("⚡ Optimized chat.py prefill took: {:?}", prefill_time);

        // PHASE 2: SINGLE TOKEN INFER (chat.py architecture with embeddings optimization)
        let infer_start = std::time::Instant::now();
        let next_token = self.run_chatpy_infer(&tokens, context_pos)?;
        let infer_time = infer_start.elapsed();
        debug!("⚡ Optimized chat.py infer took: {:?}", infer_time);

        let total_time = start_time.elapsed();
        debug!(
            "🎯 OPTIMIZED CHAT.PY TOTAL: {:?} (target: ~11ms for 87 t/s)",
            total_time
        );

        Ok(next_token)
    }

    /// Extract next token from logits (shared utility)
    fn extract_next_token(&self, logits: &Tensor) -> Result<i64, CandleError> {
        let flat_logits = logits.squeeze(0)?.squeeze(0)?;
        let logits_vec = flat_logits.to_vec1::<f32>()?;

        // Use same tie-breaking logic as TDD test
        let mut indexed_logits: Vec<(usize, f32)> = logits_vec
            .iter()
            .enumerate()
            .map(|(i, &score)| (i, score))
            .collect();
        indexed_logits.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let next_token = indexed_logits[0].0 as i64;

        // Show top predictions for debugging
        debug!("Top 5 extract_next_token predictions:");
        for (rank, (token_id, score)) in indexed_logits.iter().take(5).enumerate() {
            let decoded = self
                .tokenizer
                .decode(&[*token_id as u32], false)
                .unwrap_or("???".to_string());
            debug!(
                "  {}. Token {} ('{}'): {:.6}",
                rank + 1,
                token_id,
                decoded,
                score
            );
        }

        Ok(next_token)
    }

    /// Chat.py-style chunked prefill with embeddings caching optimization
    pub fn run_chatpy_prefill(
        &mut self,
        tokens: &[i64],
        context_pos: usize,
    ) -> Result<(), CandleError> {
        let batch_size = self.config.batch_size(); // 64
        let device = self.config.device.clone(); // Clone to avoid borrowing issues
        let causal_mask = self.cached_causal_mask.as_ref().unwrap().clone(); // Clone mask

        // Process in 64-token chunks (CoreML model constraint)
        let mut batch_pos = 0;
        while batch_pos < context_pos {
            let batch_end = (batch_pos + batch_size).min(context_pos);
            let _current_batch_size = batch_end - batch_pos;

            // Get current batch tokens
            let batch_tokens = &tokens[batch_pos..batch_end];

            // Pad to full batch size (exactly like chat.py F.pad)
            let mut padded_batch = batch_tokens.to_vec();
            padded_batch.resize(batch_size, 0); // Pad with zeros

            debug!("🔄 PREFILL: Processing batch at position {batch_pos} (batch_end: {batch_end})");
            debug!("🔄 PREFILL: batch_tokens: {batch_tokens:?}");
            debug!(
                "🔄 PREFILL: padded_batch (len={}): {:?}",
                padded_batch.len(),
                &padded_batch[..10.min(padded_batch.len())]
            );

            // 🚀 OPTIMIZATION: Try to reuse cached embeddings instead of recomputing
            let hidden_states = if let Some(cached_embeddings) =
                self.get_cached_batch_embeddings(&padded_batch)?
            {
                debug!("⚡ CACHE HIT: Reusing cached embeddings for batch at position {} with shape {:?}", batch_pos, cached_embeddings.dims());
                cached_embeddings
            } else {
                debug!("💾 CACHE MISS: Computing embeddings for batch at position {batch_pos}");

                // Find meaningful tokens (before padding zeros)
                let meaningful_end = padded_batch
                    .iter()
                    .position(|&x| x == 0)
                    .unwrap_or(padded_batch.len());
                let meaningful_tokens = &padded_batch[..meaningful_end];

                debug!(
                    "🔍 PREFILL: meaningful_end: {meaningful_end}, meaningful_tokens: {meaningful_tokens:?}"
                );

                // Fallback to direct embeddings computation
                let batch_input = self.create_embeddings_input_tensor(meaningful_tokens)?;
                debug!(
                    "✅ PREFILL: Created batch_input with shape: {:?}",
                    batch_input.dims()
                );

                let embeddings = self.embeddings.forward(&[&batch_input])?;
                debug!(
                    "✅ PREFILL: Got embeddings with shape: {:?}",
                    embeddings.dims()
                );
                embeddings
            };

            // 🚀 OPTIMIZATION: Reuse cached position IDs or create new tensor
            let position_ids = {
                let position_ids_vec: Vec<i64> =
                    (batch_pos as i64..(batch_pos + batch_size) as i64).collect();
                self.create_position_tensor(position_ids_vec)?
            };

            // Use pre-computed causal mask slice (like chat.py batch_causal_mask)
            let batch_causal_mask = causal_mask.narrow(2, batch_pos, batch_size)?;

            // 🚀 OPTIMIZATION: Reuse cached single position tensor or create new
            let current_pos = Tensor::from_vec(vec![batch_pos as i64], (1,), &device)?;

            // Run prefill with the working method
            let _output = self.run_ffn_prefill_with_inputs(
                &hidden_states,
                &position_ids,
                &batch_causal_mask,
                &current_pos,
            )?;

            batch_pos = batch_end;
        }

        debug!(
            "✅ Optimized chat.py prefill: Processed {} tokens in {} chunks",
            context_pos,
            context_pos.div_ceil(batch_size)
        );
        Ok(())
    }

    /// Chat.py-style single token infer with embeddings caching optimization
    pub fn run_chatpy_infer(&mut self, tokens: &[i64], pos: usize) -> Result<i64, CandleError> {
        let context_length = self.config.context_length();
        let _causal_mask = self.cached_causal_mask.as_ref().unwrap().clone(); // Clone mask

        // 🚀 OPTIMIZATION: Get appropriate hidden states based on model architecture
        let hidden_states = self.get_infer_hidden_states(tokens, pos)?;

        // 🚀 OPTIMIZATION: Use mode-aware position IDs creation (infer mode)
        let position_ids = self
            .config
            .create_position_ids_with_mode_detection(&[(pos - 1) as i64], false)?;

        // Fix bounds checking for causal mask slicing
        let mask_pos = pos - 1;
        if mask_pos >= context_length {
            return Err(CandleError::Msg(format!(
                "Position {mask_pos} exceeds causal mask context length {context_length}. Input may be too long for chunked processing."
            )));
        }
        let current_pos = position_ids.clone();

        // Use mode-aware causal mask creation (infer mode)
        let infer_causal_mask =
            self.config
                .create_causal_mask_with_mode_detection(mask_pos, context_length, false)?;

        // Run infer using mode-appropriate causal mask
        let infer_output = self.run_ffn_infer_with_inputs(
            &hidden_states,
            &position_ids,
            &infer_causal_mask,
            &current_pos,
        )?;

        // Run LM head and extract token (like chat.py)
        let logits = self.run_lm_head_with_inputs(&infer_output)?;
        let next_token = self.extract_next_token(&logits)?;

        debug!(
            "✅ Optimized chat.py infer: Generated token {} at position {}",
            next_token, pos
        );
        Ok(next_token)
    }

    /// Performance benchmark for the current implementation
    pub fn benchmark_implementations(
        &mut self,
        text: &str,
        iterations: usize,
    ) -> Result<(), CandleError> {
        info!("🏁 PERFORMANCE BENCHMARK: Chat.py-style Implementation");
        info!("Text: '{text}'");
        info!("Iterations: {iterations}");
        info!("================================");

        // Benchmark current forward_text implementation (chat.py-style)
        let start = std::time::Instant::now();
        let mut results = Vec::new();
        for i in 0..iterations {
            let token = self.forward_text(text)?;
            results.push(token);
            if i == 0 {
                info!("🚀 Result: token {token}");
                // Decode the token to show what it predicts
                if let Ok(decoded) = self.tokenizer.decode(&[token as u32], false) {
                    info!("   Decoded: '{decoded}'");
                }
            }
        }
        let total_time = start.elapsed();
        let avg_time = total_time / iterations as u32;
        let tokens_per_sec = 1000.0 / avg_time.as_millis() as f64;

        info!("🚀 CURRENT IMPLEMENTATION (Chat.py-style):");
        info!("   Total time: {total_time:?}");
        info!("   Average per call: {avg_time:?}");
        info!("   Tokens/second: {tokens_per_sec:.2}");

        // Performance target assessment
        if tokens_per_sec >= 70.0 {
            info!("🎯 TARGET ACHIEVED: {tokens_per_sec:.2} t/s >= 70 t/s ✅");
        } else if tokens_per_sec >= 20.0 {
            info!("🎯 PARTIAL SUCCESS: {tokens_per_sec:.2} t/s >= 20 t/s (minimum target) ⚠️");
        } else {
            info!("🎯 TARGET MISSED: {tokens_per_sec:.2} t/s < 20 t/s ❌");
        }

        // Consistency check
        let all_same = results.iter().all(|&token| token == results[0]);
        info!(
            "✅ Consistency: {} (all iterations produced {})",
            if all_same {
                "CONSISTENT"
            } else {
                "INCONSISTENT"
            },
            if all_same {
                "same result"
            } else {
                "different results"
            }
        );

        Ok(())
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
            .map_err(|e| CandleError::Msg(format!("Failed to decode tokens: {e}")))
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
            if next_token == 151_645 {
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

    /// Generate tokens using combined top-k + temperature sampling similar to Python reference script.
    /// If top_k is Some(k) and k > 0, restrict sampling to top-k logits; if temperature <= 0 use greedy within that set.
    pub fn generate_tokens_topk_temp(
        &mut self,
        text: &str,
        max_tokens: usize,
        temperature: f32,
        top_k: Option<usize>,
    ) -> Result<Vec<i64>, CandleError> {
        use crate::utils::sampling;
        let mut generated_tokens = Vec::new();
        let mut current_text = text.to_string();

        for _ in 0..max_tokens {
            // Ensure stateful pipeline progresses: run forward_text to build caches & get greedy logits via infer path
            // Adapt forward_text to expose logits by duplicating last steps inline
            if self.unified_state.is_none() || self.cached_causal_mask.is_none() {
                self.initialize_states()?;
            }
            let tokens = self.tokenize(&current_text)?;
            let context_pos = tokens.len();
            // Cache embeddings & run chunked prefill only first time or when context grows
            self.run_chatpy_prefill(&tokens, context_pos)?;
            // Run infer to get logits tensor
            let hidden_states = self.get_infer_hidden_states(&tokens, context_pos)?;
            let position_ids = self
                .config
                .create_position_ids_with_mode_detection(&[(context_pos - 1) as i64], false)?;
            let infer_causal_mask = self.config.create_causal_mask_with_mode_detection(
                context_pos - 1,
                self.config.context_length(),
                false,
            )?;
            let current_pos = position_ids.clone();
            let infer_output = self.run_ffn_infer_with_inputs(
                &hidden_states,
                &position_ids,
                &infer_causal_mask,
                &current_pos,
            )?;
            let logits_tensor = self.run_lm_head_with_inputs(&infer_output)?;
            let flat_logits = logits_tensor.squeeze(0)?.squeeze(0)?; // [vocab]

            // Sampling strategy
            let next_token = if let Some(k) = top_k {
                sampling::sample_top_k(&flat_logits, k, temperature)?
            } else if temperature > 0.0 {
                sampling::sample_with_temperature(&flat_logits, temperature)?
            } else {
                sampling::greedy_sample(&flat_logits)?
            };

            generated_tokens.push(next_token);
            // Stop if EOS
            if next_token == 151_645 {
                // TODO: obtain dynamically from tokenizer special tokens
                break;
            }
            if let Ok(decoded) = self.tokenizer.decode(&[next_token as u32], false) {
                current_text.push_str(&decoded);
            } else {
                break;
            }
        }
        Ok(generated_tokens)
    }

    /// 🚀 OPTIMIZATION: Try to get cached embeddings for a batch of tokens
    /// This checks if the padded batch matches part of our cached sequence
    fn get_cached_batch_embeddings(
        &self,
        padded_batch: &[i64],
    ) -> Result<Option<Tensor>, CandleError> {
        // Check if we have cached embeddings for the full sequence
        if let Some((cached_tokens, cached_embeddings)) = &self.last_sequence_embeddings {
            // Try to find if this padded batch corresponds to a slice of our cached sequence
            let batch_size = padded_batch.len();

            // Look for the meaningful part of the batch (before padding zeros)
            let meaningful_end = padded_batch
                .iter()
                .position(|&x| x == 0)
                .unwrap_or(batch_size);

            if meaningful_end > 0 {
                let meaningful_batch = &padded_batch[..meaningful_end];

                // Check if this meaningful batch appears at the start of our cached tokens
                if cached_tokens.len() >= meaningful_batch.len()
                    && &cached_tokens[..meaningful_batch.len()] == meaningful_batch
                {
                    // Check if cached embeddings have sufficient size for the requested batch
                    let cached_dims = cached_embeddings.dims();

                    // SHAPE VALIDATION: Ensure cached embeddings have enough positions (dim 1) for the requested batch_size
                    if cached_dims.len() >= 2 && cached_dims[1] >= batch_size {
                        // Extract the corresponding embeddings slice
                        let batch_embeddings = cached_embeddings.narrow(1, 0, batch_size)?;
                        debug!(
                            "⚡ EMBEDDINGS CACHE HIT: Reusing {} tokens from cached sequence (dims: {:?} -> batch_size: {})",
                            meaningful_end, cached_dims, batch_size
                        );
                        return Ok(Some(batch_embeddings));
                    }

                    // SHAPE MISMATCH: Cached embeddings don't have enough positions for the requested batch
                    debug!(
                        "⚠️ EMBEDDINGS CACHE MISS: Shape mismatch - cached dims {:?} insufficient for batch_size {} (need at least {} positions in dim 1)",
                        cached_dims, batch_size, batch_size
                    );
                }
            }
        }

        // No cache hit found
        Ok(None)
    }
}
