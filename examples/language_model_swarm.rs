use wingbeat::{EnhancedProcessor, TornadoSwarm, DecompositionStrategy};
use std::sync::Arc;
use colored::*;

/// Example demonstrating language model decomposition and swarm processing
#[tokio::main]
async fn main() {
    println!("{}", "╔══════════════════════════════════════════╗".bright_cyan());
    println!("{}", "║    LANGUAGE MODEL SWARM PROCESSING       ║".bright_cyan().bold());
    println!("{}", "╚══════════════════════════════════════════╝".bright_cyan());
    println!();
    
    // Create the tornado swarm
    let swarm = Arc::new(TornadoSwarm::new());
    
    // Create enhanced processor
    let mut processor = EnhancedProcessor::new(swarm);
    
    // Example prompts to process
    let prompts = vec![
        "Explain quantum computing in simple terms",
        "Write a haiku about artificial intelligence",
        "What is the meaning of life?",
    ];
    
    let strategies = vec![
        DecompositionStrategy::LayerWise,
        DecompositionStrategy::AttentionHeads,
        DecompositionStrategy::TokenWise,
    ];
    
    for (i, (prompt, strategy)) in prompts.iter().zip(strategies.iter()).enumerate() {
        println!("{}", format!("═══ Processing Example {} ═══", i + 1).bright_blue().bold());
        println!("{}", format!("Strategy: {:?}", strategy).yellow());
        println!("{}", format!("Prompt: {}", prompt).bright_green());
        println!();
        
        // Process the prompt through the decomposed model
        let result = processor.process_with_model(prompt, strategy.clone()).await;
        
        println!();
        println!("{}", "───────────────────────────────────────────".bright_black());
        println!();
    }
    
    println!("{}", "🎉 Language model swarm processing complete!".bright_green().bold());
    println!();
    
    // Show summary of what was accomplished
    println!("{}", "📊 Processing Summary:".bright_yellow().bold());
    println!("  • 3 different decomposition strategies tested");
    println!("  • Language model layers distributed across tornado swarm");
    println!("  • Subgraphs computed in parallel across distributed nodes");
    println!("  • Results reintegrated into coherent model outputs");
    println!("  • Swarm dynamics maintained throughout processing");
    println!();
    
    println!("{}", "🚀 This demonstrates the core Wingbeat vision:".bright_cyan());
    println!("  • Take a language model prompt");
    println!("  • Disintegrate the model into subgraphs");
    println!("  • Distribute computation across a swarm");
    println!("  • Reintegrate results into a complete response");
} 