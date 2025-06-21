pub mod tensor_ops;
pub mod model_weights;
pub mod layer_ops;
pub mod tokenizer;

pub use tensor_ops::{Tensor, DataType, TensorOps};
pub use model_weights::{ModelWeights, ModelParameter};
pub use layer_ops::{LayerOperation, LayerContext, LayerResult, LayerFactory};
pub use tokenizer::{Tokenizer, Token, SimpleTokenizer, BPETokenizer, TokenizerFactory};

/// Inference engine that coordinates all components
#[derive(Debug)]
pub struct InferenceEngine {
    pub weights: ModelWeights,
    pub tokenizer: Box<dyn Tokenizer>,
    pub layer_operations: Vec<Box<dyn LayerOperation>>,
}

impl InferenceEngine {
    pub fn new(tokenizer_type: &str) -> Self {
        Self {
            weights: ModelWeights::new(),
            tokenizer: TokenizerFactory::create_tokenizer(tokenizer_type),
            layer_operations: Vec::new(),
        }
    }

    /// Initialize a basic transformer model
    pub fn init_transformer(&mut self, num_layers: usize, hidden_size: usize, vocab_size: usize) {
        use crate::computation::model_decomposer::LayerType;
        use uuid::Uuid;
        use std::collections::HashMap;

        // Initialize weights for each layer
        for layer_idx in 0..num_layers {
            let layer_id = Uuid::new_v4();
            self.weights.init_transformer_layer(layer_id, hidden_size, vocab_size);
            
            // Create layer operations
            let config = HashMap::from([
                ("hidden_size".to_string(), hidden_size),
                ("vocab_size".to_string(), vocab_size),
                ("num_heads".to_string(), 12),
            ]);

            // Add embedding layer (only for first layer)
            if layer_idx == 0 {
                let embedding_layer = LayerFactory::create_layer(
                    LayerType::Embedding,
                    layer_id,
                    config.clone(),
                );
                self.layer_operations.push(embedding_layer);
            }

            // Add attention layer
            let attention_layer = LayerFactory::create_layer(
                LayerType::Attention,
                layer_id,
                config.clone(),
            );
            self.layer_operations.push(attention_layer);

            // Add feedforward layer
            let ffn_layer = LayerFactory::create_layer(
                LayerType::FeedForward,
                layer_id,
                config.clone(),
            );
            self.layer_operations.push(ffn_layer);
        }

        // Add output layer
        let output_layer_id = Uuid::new_v4();
        let output_config = HashMap::from([
            ("hidden_size".to_string(), hidden_size),
            ("vocab_size".to_string(), vocab_size),
        ]);
        let output_layer = LayerFactory::create_layer(
            LayerType::Output,
            output_layer_id,
            output_config,
        );
        self.layer_operations.push(output_layer);
    }

    /// Run inference on a text input
    pub fn infer(&self, text: &str) -> Result<String, String> {
        // Tokenize input
        let tokens = self.tokenizer.encode(text)?;
        
        // Convert tokens to tensor
        let token_ids: Vec<f32> = tokens.iter().map(|t| t.id as f32).collect();
        let mut input_tensor = Tensor::new(vec![1, token_ids.len()], token_ids);
        
        // Run through all layers
        for layer_op in &self.layer_operations {
            let context = LayerContext {
                input: input_tensor.clone(),
                output: None,
                metadata: HashMap::new(),
            };
            
            let result = layer_op.execute(context, &self.weights)?;
            input_tensor = result.output;
        }
        
        // Convert output back to tokens (simplified)
        let output_tokens = self.tensor_to_tokens(&input_tensor)?;
        
        // Decode tokens back to text
        self.tokenizer.decode(&output_tokens)
    }

    /// Convert output tensor back to tokens (simplified)
    fn tensor_to_tokens(&self, tensor: &Tensor) -> Result<Vec<Token>, String> {
        // This is a simplified conversion - in practice you'd do proper sampling
        let mut tokens = Vec::new();
        
        for (i, &logit) in tensor.data.iter().enumerate() {
            let token_id = logit as u32 % self.tokenizer.vocab_size() as u32;
            tokens.push(Token {
                id: token_id,
                text: format!("token_{}", token_id),
                start: i,
                end: i + 1,
            });
        }
        
        Ok(tokens)
    }

    /// Get model statistics
    pub fn get_stats(&self) -> HashMap<String, String> {
        HashMap::from([
            ("total_parameters".to_string(), self.weights.parameter_count().to_string()),
            ("num_layers".to_string(), (self.layer_operations.len() / 3).to_string()), // Rough estimate
            ("vocab_size".to_string(), self.tokenizer.vocab_size().to_string()),
        ])
    }
} 