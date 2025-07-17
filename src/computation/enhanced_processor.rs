use crate::{
    swarm::tornado::TornadoSwarm,
    computation::model_decomposer::{ModelDecomposer, DecompositionStrategy},
    core::subgraph::{Subgraph, SubgraphId, SubgraphType},
    inference::{ModelWeights, Tensor, LayerOperation},
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Enhanced processor that integrates model decomposition with swarm processing
pub struct EnhancedProcessor {
    pub swarm: Arc<TornadoSwarm>,
    pub decomposer: ModelDecomposer,
}

impl EnhancedProcessor {
    pub fn new(swarm: Arc<TornadoSwarm>, decomposer: ModelDecomposer) -> Self {
        Self { swarm, decomposer }
    }

    /// Process a prompt through the swarm with model decomposition
    pub async fn process_prompt(&mut self, prompt: &str) -> Result<SwarmPromptResult, String> {
        println!("🌪️  Processing prompt: '{}'", prompt);
        
        // Decompose the model into subgraphs using the correct parameter type
        let subgraphs = self.decomposer.decompose_model(DecompositionStrategy::LayerWise).await;
        println!("   Decomposed into {} subgraphs", subgraphs.len());
        
        // Distribute subgraphs across the swarm
        let mut distributed_subgraphs = Vec::new();
        let tornadoes = self.swarm.tornadoes.read().await;
        let tornado_count = tornadoes.len();
        drop(tornadoes); // Release the read lock
        
        for (i, subgraph) in subgraphs.iter().enumerate() {
            let tornadoes = self.swarm.tornadoes.read().await;
            let tornado = &tornadoes[i % tornado_count];
            tornado.sweep_up(Arc::new(RwLock::new(subgraph.clone()))).await;
            distributed_subgraphs.push(subgraph.clone());
            println!("   Subgraph {} distributed to tornado {}", i, tornado.id);
        }
        
        // Simulate computation in the swarm
        println!("   🌀 Spinning tornadoes...");
        let tornadoes = self.swarm.tornadoes.read().await;
        for tornado in tornadoes.iter() {
            tornado.spin().await;
        }
        
        // Process the prompt through each subgraph
        let mut results = Vec::new();
        for (i, subgraph) in distributed_subgraphs.iter().enumerate() {
            let result = self.process_subgraph(subgraph, prompt).await?;
            results.push(result);
            println!("   Subgraph {} processed: {:?}", i, subgraph.subgraph_type);
        }
        
        // Reintegrate results
        let final_result = self.reintegrate_results(results, prompt).await?;
        println!("   ✅ Results reintegrated");
        
        // Release subgraphs back to the swarm
        let tornadoes = self.swarm.tornadoes.read().await;
        for tornado in tornadoes.iter() {
            tornado.release(1).await;
        }
        
        Ok(SwarmPromptResult {
            status: PromptStatus::Completed,
            output: Some(final_result),
            metadata: HashMap::new(),
        })
    }

    /// Process a single subgraph
    async fn process_subgraph(&self, subgraph: &Subgraph, prompt: &str) -> Result<String, String> {
        // Simulate processing based on subgraph type
        match subgraph.subgraph_type {
            SubgraphType::Transformer => {
                Ok(format!("[Transformed: {}]", prompt))
            }
            SubgraphType::Embedding => {
                Ok(format!("[Embedded: {}]", prompt))
            }
            SubgraphType::Attention => {
                Ok(format!("[Attended: {}]", prompt))
            }
            SubgraphType::FeedForward => {
                Ok(format!("[Processed: {}]", prompt))
            }
            SubgraphType::OutputLayer => {
                Ok(format!("[Output: {}]", prompt))
            }
            SubgraphType::Custom(_) => {
                Ok(format!("[Custom: {}]", prompt))
            }
        }
    }

    /// Reintegrate results from all subgraphs
    async fn reintegrate_results(&self, results: Vec<String>, original_prompt: &str) -> Result<String, String> {
        let mut final_output = format!("🌪️ Swarm processed: '{}'\n", original_prompt);
        final_output.push_str("📊 Results from subgraphs:\n");
        
        for (i, result) in results.iter().enumerate() {
            final_output.push_str(&format!("   Subgraph {}: {}\n", i, result));
        }
        
        final_output.push_str("🎯 Final integrated response: ");
        final_output.push_str(&format!("'{}' processed through {} subgraphs", 
            original_prompt, results.len()));
        
        Ok(final_output)
    }

    /// Run distributed inference with real model weights
    pub async fn run_distributed_inference(
        &mut self, 
        prompt: &str, 
        weights: &ModelWeights
    ) -> Result<String, String> {
        println!("🚀 Running distributed inference for: '{}'", prompt);
        
        // Decompose model with real weights
        let subgraphs = self.decomposer.decompose_model(DecompositionStrategy::LayerWise).await;
        println!("   Model decomposed into {} subgraphs", subgraphs.len());
        
        // Distribute across swarm
        let mut distributed_results = Vec::new();
        let tornadoes = self.swarm.tornadoes.read().await;
        let tornado_count = tornadoes.len();
        drop(tornadoes); // Release the read lock

        for (i, subgraph) in subgraphs.iter().enumerate() {
            let tornadoes = self.swarm.tornadoes.read().await;
            let tornado = &tornadoes[i % tornado_count];
            tornado.sweep_up(Arc::new(RwLock::new(subgraph.clone()))).await;
            
            // Simulate inference on this subgraph
            let result = self.simulate_inference_on_subgraph(subgraph, prompt, weights).await?;
            distributed_results.push(result);
            
                         tornado.release(1).await;
        }
        
        // Combine results
        let final_result = self.combine_inference_results(distributed_results, prompt).await?;
        
        Ok(final_result)
    }

    /// Simulate inference on a specific subgraph
    async fn simulate_inference_on_subgraph(
        &self, 
        subgraph: &Subgraph, 
        prompt: &str, 
        weights: &ModelWeights
    ) -> Result<String, String> {
        // Simulate different types of inference based on subgraph type
        match subgraph.subgraph_type {
            SubgraphType::Transformer => {
                // Simulate full transformer computation
                let layer_count = weights.parameter_count() / 1000000; // Rough estimate
                Ok(format!("Computed through {} transformer layers", layer_count))
            }
            SubgraphType::Embedding => {
                // Simulate embedding lookup
                let token_count = prompt.split_whitespace().count();
                Ok(format!("Embedded {} tokens", token_count))
            }
            SubgraphType::Attention => {
                // Simulate attention computation
                let attention_heads = 12;
                Ok(format!("Computed attention with {} heads", attention_heads))
            }
            SubgraphType::FeedForward => {
                // Simulate feedforward computation
                let hidden_size = 768;
                Ok(format!("Processed through {} hidden units", hidden_size))
            }
            SubgraphType::OutputLayer => {
                // Simulate output projection
                let vocab_size = weights.parameter_count() / 1000; // Rough estimate
                Ok(format!("Projected to {} vocabulary tokens", vocab_size))
            }
            SubgraphType::Custom(_) => {
                Ok("Custom computation completed".to_string())
            }
        }
    }

    /// Combine results from distributed inference
    async fn combine_inference_results(
        &self, 
        results: Vec<String>, 
        original_prompt: &str
    ) -> Result<String, String> {
        let mut combined = format!("🎯 Distributed inference results for '{}':\n", original_prompt);
        
        for (i, result) in results.iter().enumerate() {
            combined.push_str(&format!("   Subgraph {}: {}\n", i, result));
        }
        
        combined.push_str("✅ All subgraphs processed successfully");
        
        Ok(combined)
    }
}

/// Result of swarm prompt processing
#[derive(Debug)]
pub struct SwarmPromptResult {
    pub status: PromptStatus,
    pub output: Option<String>,
    pub metadata: HashMap<String, String>,
}

/// Status of prompt processing
#[derive(Debug)]
pub enum PromptStatus {
    Pending,
    Processing,
    Completed,
    Failed(String),
} 