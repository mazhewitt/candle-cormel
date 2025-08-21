//! Generic shape inference from CoreML components
//!
//! Infers model dimensions (batch size, hidden size, etc.) from actual tensor configurations

use crate::model_config::{ComponentConfig, ShapeConfig};
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

    /// Compute overall shape configuration from components (generic approach)
    pub fn infer_shapes(&self, components: &HashMap<String, ComponentConfig>) -> ShapeConfig {
        let batch_size = self.infer_batch_size(components);
        let hidden_size = self.infer_hidden_size(components);
        let context_length = self.infer_context_length(components);
        let vocab_size = self.infer_vocab_size(components);

        debug!(
            "📊 Inferred shapes: batch={}, context={}, hidden={}, vocab={}",
            batch_size, context_length, hidden_size, vocab_size
        );

        ShapeConfig {
            batch_size,
            context_length,
            hidden_size,
            vocab_size,
        }
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

    /// Infer hidden size from the largest feature dimension
    fn infer_hidden_size(&self, components: &HashMap<String, ComponentConfig>) -> usize {
        let mut feature_sizes = Vec::new();

        for component in components.values() {
            for tensor in component.inputs.values().chain(component.outputs.values()) {
                if tensor.shape.len() >= 3 {
                    // 3D tensors: [batch, seq, feature]
                    feature_sizes.push(tensor.shape[2]);
                } else if tensor.shape.len() == 2 && tensor.shape[1] > 100 {
                    // Could be a flattened feature vector
                    feature_sizes.push(tensor.shape[1]);
                }
            }
        }

        feature_sizes.into_iter().max().unwrap_or(1024)
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

    /// Infer vocabulary size from the largest output dimension
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
