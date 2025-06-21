use wingbeat::{
    InferenceEngine, TornadoSwarm, EnhancedProcessor,
    ModelDecomposer, DecompositionStrategy,
    Tensor, LayerOperation,
};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌪️  Wingbeat Real Inference Demo");
    println!("================================\n");

    // Initialize the inference engine
    println!("🔧 Initializing inference engine...");
    let mut engine = InferenceEngine::new("simple");
    engine.init_transformer(6, 768, 51200); // 6 layers, 768 hidden size, 50k vocab
    
    let stats = engine.get_stats();
    println!("📊 Model Stats:");
    for (key, value) in &stats {
        println!("   {}: {}", key, value);
    }
    println!();

    // Initialize the swarm
    println!("🌪️  Initializing tornado swarm...");
    let mut swarm = TornadoSwarm::new(8); // 8 tornadoes
    swarm.initialize();
    println!("   Swarm initialized with {} tornadoes", swarm.tornadoes.len());
    println!();

    // Create model decomposer
    println!("🔍 Creating model decomposer...");
    let decomposer = ModelDecomposer::new(
        "transformer".to_string(),
        DecompositionStrategy::LayerWise,
        HashMap::new(),
    );
    println!("   Decomposer created with {:?} strategy", DecompositionStrategy::LayerWise);
    println!();

    // Create enhanced processor
    println!("⚡ Creating enhanced processor...");
    let mut processor = EnhancedProcessor::new(swarm, decomposer);
    println!("   Enhanced processor ready");
    println!();

    // Test basic tensor operations
    println!("🧮 Testing tensor operations...");
    let tensor_a = Tensor::new(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let tensor_b = Tensor::new(vec![2, 3], vec![0.5, 1.0, 1.5, 2.0, 2.5, 3.0]);
    
    let result = tensor_a.add(&tensor_b)?;
    println!("   Tensor addition: {:?}", result.data);
    
    let relu_result = tensor_a.relu();
    println!("   ReLU activation: {:?}", relu_result.data);
    println!();

    // Test tokenization
    println!("🔤 Testing tokenization...");
    let text = "Hello world from Wingbeat";
    let tokens = engine.tokenizer.encode(text)?;
    println!("   Input text: '{}'", text);
    println!("   Tokens: {:?}", tokens.iter().map(|t| &t.text).collect::<Vec<_>>());
    println!();

    // Test layer operations
    println!("🏗️  Testing layer operations...");
    let test_input = Tensor::new(vec![1, 4], vec![1.0, 2.0, 3.0, 4.0]);
    
    for (i, layer_op) in engine.layer_operations.iter().enumerate() {
        let context = wingbeat::LayerContext {
            input: test_input.clone(),
            output: None,
            metadata: HashMap::new(),
        };
        
        match layer_op.execute(context, &engine.weights) {
            Ok(result) => {
                println!("   Layer {} ({:?}): Output shape {:?}", 
                    i, layer_op.layer_type(), result.output.shape);
            }
            Err(e) => {
                println!("   Layer {} ({:?}): Error - {}", 
                    i, layer_op.layer_type(), e);
            }
        }
    }
    println!();

    // Test swarm processing with real inference
    println!("🌪️  Testing swarm processing with real inference...");
    let test_prompts = vec![
        "Hello world",
        "The quick brown fox",
        "Wingbeat is amazing",
    ];

    for (i, prompt) in test_prompts.iter().enumerate() {
        println!("   Processing prompt {}: '{}'", i + 1, prompt);
        
        // Process through the swarm
        let result = processor.process_prompt(prompt).await?;
        
        println!("   Swarm result: {:?}", result.status);
        if let Some(output) = result.output {
            println!("   Output: {}", output);
        }
        println!();
    }

    // Test model decomposition with real weights
    println!("🔍 Testing model decomposition with real weights...");
    let subgraphs = processor.decomposer.decompose_model(&engine.weights)?;
    println!("   Decomposed into {} subgraphs", subgraphs.len());
    
    for (i, subgraph) in subgraphs.iter().enumerate() {
        println!("   Subgraph {}: {:?} with {} nodes", 
            i, subgraph.subgraph_type, subgraph.nodes.len());
    }
    println!();

    // Test distributed inference simulation
    println!("🚀 Testing distributed inference simulation...");
    let distributed_result = processor.run_distributed_inference(
        "Distributed inference test",
        &engine.weights,
    ).await?;
    
    println!("   Distributed inference completed");
    println!("   Final output: {}", distributed_result);
    println!();

    println!("✅ Real inference demo completed successfully!");
    println!("\n🎯 Next steps for production:");
    println!("   1. Integrate with real model weights (HuggingFace, etc.)");
    println!("   2. Add proper attention mechanisms");
    println!("   3. Implement efficient tensor operations (BLAS, etc.)");
    println!("   4. Add proper tokenization (BPE, SentencePiece, etc.)");
    println!("   5. Implement proper sampling strategies");
    println!("   6. Add model checkpointing and loading");
    println!("   7. Optimize for GPU/TPU acceleration");

    Ok(())
} 