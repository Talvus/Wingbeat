use crate::inference::tensor_ops::Tensor;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Represents a model parameter/weight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelParameter {
    pub id: Uuid,
    pub name: String,
    pub tensor: Tensor,
    pub requires_grad: bool,
    pub layer_id: Uuid,
}

/// Manages model weights and parameters
#[derive(Debug)]
pub struct ModelWeights {
    pub parameters: HashMap<String, ModelParameter>,
    pub layer_parameters: HashMap<Uuid, Vec<String>>,
}

impl ModelWeights {
    pub fn new() -> Self {
        Self {
            parameters: HashMap::new(),
            layer_parameters: HashMap::new(),
        }
    }

    /// Add a parameter to the model
    pub fn add_parameter(&mut self, name: String, tensor: Tensor, layer_id: Uuid) {
        let param = ModelParameter {
            id: Uuid::new_v4(),
            name: name.clone(),
            tensor,
            requires_grad: true,
            layer_id,
        };
        
        self.parameters.insert(name.clone(), param);
        
        // Track parameters by layer
        self.layer_parameters
            .entry(layer_id)
            .or_insert_with(Vec::new)
            .push(name);
    }

    /// Get a parameter by name
    pub fn get_parameter(&self, name: &str) -> Option<&ModelParameter> {
        self.parameters.get(name)
    }

    /// Get all parameters for a layer
    pub fn get_layer_parameters(&self, layer_id: Uuid) -> Vec<&ModelParameter> {
        if let Some(param_names) = self.layer_parameters.get(&layer_id) {
            param_names.iter()
                .filter_map(|name| self.parameters.get(name))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Initialize weights for a typical transformer layer
    pub fn init_transformer_layer(&mut self, layer_id: Uuid, hidden_size: usize, vocab_size: usize) {
        // Embedding weights
        self.add_parameter(
            "embedding.weight".to_string(),
            Tensor::random(vec![vocab_size, hidden_size]),
            layer_id,
        );

        // Attention weights
        self.add_parameter(
            "attention.query.weight".to_string(),
            Tensor::random(vec![hidden_size, hidden_size]),
            layer_id,
        );
        
        self.add_parameter(
            "attention.key.weight".to_string(),
            Tensor::random(vec![hidden_size, hidden_size]),
            layer_id,
        );
        
        self.add_parameter(
            "attention.value.weight".to_string(),
            Tensor::random(vec![hidden_size, hidden_size]),
            layer_id,
        );
        
        self.add_parameter(
            "attention.output.weight".to_string(),
            Tensor::random(vec![hidden_size, hidden_size]),
            layer_id,
        );

        // Feedforward weights
        self.add_parameter(
            "ffn.intermediate.weight".to_string(),
            Tensor::random(vec![hidden_size, hidden_size * 4]),
            layer_id,
        );
        
        self.add_parameter(
            "ffn.output.weight".to_string(),
            Tensor::random(vec![hidden_size * 4, hidden_size]),
            layer_id,
        );

        // Layer norm weights
        self.add_parameter(
            "attention_norm.weight".to_string(),
            Tensor::ones(vec![hidden_size]),
            layer_id,
        );
        
        self.add_parameter(
            "ffn_norm.weight".to_string(),
            Tensor::ones(vec![hidden_size]),
            layer_id,
        );
    }

    /// Save weights to a file (simplified)
    pub fn save(&self, path: &str) -> Result<(), String> {
        // This would serialize to a file in practice
        println!("Saving weights to {}", path);
        Ok(())
    }

    /// Load weights from a file (simplified)
    pub fn load(&mut self, path: &str) -> Result<(), String> {
        // This would deserialize from a file in practice
        println!("Loading weights from {}", path);
        Ok(())
    }

    /// Get total number of parameters
    pub fn parameter_count(&self) -> usize {
        self.parameters.values()
            .map(|p| p.tensor.size())
            .sum()
    }
} 