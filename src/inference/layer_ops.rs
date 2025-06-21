use crate::inference::{tensor_ops::Tensor, model_weights::ModelWeights};
use crate::computation::model_decomposer::LayerType;
use std::collections::HashMap;
use uuid::Uuid;

/// Input/output context for layer operations
#[derive(Debug, Clone)]
pub struct LayerContext {
    pub input: Tensor,
    pub output: Option<Tensor>,
    pub metadata: HashMap<String, String>,
}

/// Result of a layer operation
#[derive(Debug)]
pub struct LayerResult {
    pub output: Tensor,
    pub metadata: HashMap<String, String>,
}

/// Trait for layer operations that can be executed
pub trait LayerOperation {
    fn execute(&self, context: LayerContext, weights: &ModelWeights) -> Result<LayerResult, String>;
    fn layer_type(&self) -> LayerType;
    fn layer_id(&self) -> Uuid;
}

/// Embedding layer operation
#[derive(Debug)]
pub struct EmbeddingLayer {
    pub layer_id: Uuid,
    pub vocab_size: usize,
    pub hidden_size: usize,
}

impl LayerOperation for EmbeddingLayer {
    fn execute(&self, context: LayerContext, weights: &ModelWeights) -> Result<LayerResult, String> {
        // Get embedding weights
        let embedding_weight = weights.get_parameter("embedding.weight")
            .ok_or("Embedding weights not found")?;
        
        // Simple embedding lookup (in practice, this would be more sophisticated)
        let input_data = &context.input.data;
        let mut output_data = Vec::new();
        
        for &token_id in input_data {
            let token_idx = token_id as usize % self.vocab_size;
            let start_idx = token_idx * self.hidden_size;
            let end_idx = start_idx + self.hidden_size;
            
            if end_idx <= embedding_weight.tensor.data.len() {
                output_data.extend_from_slice(&embedding_weight.tensor.data[start_idx..end_idx]);
            } else {
                // Pad with zeros if out of bounds
                output_data.extend(vec![0.0; self.hidden_size]);
            }
        }
        
        let output = Tensor::new(vec![input_data.len(), self.hidden_size], output_data);
        
        Ok(LayerResult {
            output,
            metadata: HashMap::from([
                ("operation".to_string(), "embedding".to_string()),
                ("vocab_size".to_string(), self.vocab_size.to_string()),
                ("hidden_size".to_string(), self.hidden_size.to_string()),
            ]),
        })
    }

    fn layer_type(&self) -> LayerType {
        LayerType::Embedding
    }

    fn layer_id(&self) -> Uuid {
        self.layer_id
    }
}

/// Attention layer operation
#[derive(Debug)]
pub struct AttentionLayer {
    pub layer_id: Uuid,
    pub hidden_size: usize,
    pub num_heads: usize,
}

impl LayerOperation for AttentionLayer {
    fn execute(&self, context: LayerContext, weights: &ModelWeights) -> Result<LayerResult, String> {
        let input = &context.input;
        
        // Get attention weights
        let query_weight = weights.get_parameter("attention.query.weight")
            .ok_or("Query weights not found")?;
        let key_weight = weights.get_parameter("attention.key.weight")
            .ok_or("Key weights not found")?;
        let value_weight = weights.get_parameter("attention.value.weight")
            .ok_or("Value weights not found")?;
        let output_weight = weights.get_parameter("attention.output.weight")
            .ok_or("Output weights not found")?;
        
        // Compute Q, K, V
        let query = input.matmul(&query_weight.tensor)?;
        let key = input.matmul(&key_weight.tensor)?;
        let value = input.matmul(&value_weight.tensor)?;
        
        // Simple attention computation (simplified)
        let attention_scores = query.matmul(&key.transpose())?;
        let attention_probs = attention_scores.softmax();
        let attention_output = attention_probs.matmul(&value)?;
        
        // Apply output projection
        let output = attention_output.matmul(&output_weight.tensor)?;
        
        Ok(LayerResult {
            output,
            metadata: HashMap::from([
                ("operation".to_string(), "attention".to_string()),
                ("hidden_size".to_string(), self.hidden_size.to_string()),
                ("num_heads".to_string(), self.num_heads.to_string()),
            ]),
        })
    }

    fn layer_type(&self) -> LayerType {
        LayerType::Attention
    }

    fn layer_id(&self) -> Uuid {
        self.layer_id
    }
}

/// Feedforward layer operation
#[derive(Debug)]
pub struct FeedForwardLayer {
    pub layer_id: Uuid,
    pub hidden_size: usize,
}

impl LayerOperation for FeedForwardLayer {
    fn execute(&self, context: LayerContext, weights: &ModelWeights) -> Result<LayerResult, String> {
        let input = &context.input;
        
        // Get feedforward weights
        let intermediate_weight = weights.get_parameter("ffn.intermediate.weight")
            .ok_or("Intermediate weights not found")?;
        let output_weight = weights.get_parameter("ffn.output.weight")
            .ok_or("Output weights not found")?;
        
        // Apply intermediate layer
        let intermediate = input.matmul(&intermediate_weight.tensor)?;
        let activated = intermediate.relu();
        
        // Apply output layer
        let output = activated.matmul(&output_weight.tensor)?;
        
        Ok(LayerResult {
            output,
            metadata: HashMap::from([
                ("operation".to_string(), "feedforward".to_string()),
                ("hidden_size".to_string(), self.hidden_size.to_string()),
            ]),
        })
    }

    fn layer_type(&self) -> LayerType {
        LayerType::FeedForward
    }

    fn layer_id(&self) -> Uuid {
        self.layer_id
    }
}

/// Output layer operation
#[derive(Debug)]
pub struct OutputLayer {
    pub layer_id: Uuid,
    pub hidden_size: usize,
    pub vocab_size: usize,
}

impl LayerOperation for OutputLayer {
    fn execute(&self, context: LayerContext, weights: &ModelWeights) -> Result<LayerResult, String> {
        let input = &context.input;
        
        // Get output projection weights (using embedding weights as output projection)
        let output_weight = weights.get_parameter("embedding.weight")
            .ok_or("Output weights not found")?;
        
        // Apply output projection
        let logits = input.matmul(&output_weight.tensor.transpose())?;
        
        Ok(LayerResult {
            output: logits,
            metadata: HashMap::from([
                ("operation".to_string(), "output".to_string()),
                ("hidden_size".to_string(), self.hidden_size.to_string()),
                ("vocab_size".to_string(), self.vocab_size.to_string()),
            ]),
        })
    }

    fn layer_type(&self) -> LayerType {
        LayerType::Output
    }

    fn layer_id(&self) -> Uuid {
        self.layer_id
    }
}

/// Factory for creating layer operations
pub struct LayerFactory;

impl LayerFactory {
    pub fn create_layer(layer_type: LayerType, layer_id: Uuid, config: HashMap<String, usize>) -> Box<dyn LayerOperation> {
        match layer_type {
            LayerType::Embedding => {
                let vocab_size = config.get("vocab_size").copied().unwrap_or(51200);
                let hidden_size = config.get("hidden_size").copied().unwrap_or(768);
                Box::new(EmbeddingLayer { layer_id, vocab_size, hidden_size })
            },
            LayerType::Attention => {
                let hidden_size = config.get("hidden_size").copied().unwrap_or(768);
                let num_heads = config.get("num_heads").copied().unwrap_or(12);
                Box::new(AttentionLayer { layer_id, hidden_size, num_heads })
            },
            LayerType::FeedForward => {
                let hidden_size = config.get("hidden_size").copied().unwrap_or(768);
                Box::new(FeedForwardLayer { layer_id, hidden_size })
            },
            LayerType::Output => {
                let hidden_size = config.get("hidden_size").copied().unwrap_or(768);
                let vocab_size = config.get("vocab_size").copied().unwrap_or(51200);
                Box::new(OutputLayer { layer_id, hidden_size, vocab_size })
            },
            LayerType::Custom(_) => {
                // Placeholder for custom layers
                let hidden_size = config.get("hidden_size").copied().unwrap_or(768);
                Box::new(FeedForwardLayer { layer_id, hidden_size })
            }
        }
    }
} 