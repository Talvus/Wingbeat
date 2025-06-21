use wingbeat::{TornadoSwarm, PromptProcessor, Vec3};
use std::sync::Arc;
use colored::*;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // Initialize tracing for better async debugging
    tracing_subscriber::fmt::init();
    
    println!("{}", "╔══════════════════════════════════════════╗".bright_cyan());
    println!("{}", "║           WINGBEAT SWARM ENGINE          ║".bright_cyan().bold());
    println!("{}", "╚══════════════════════════════════════════╝".bright_cyan());
    println!();
    
    // Initialize the tornado swarm
    let swarm = Arc::new(TornadoSwarm::new());
    
    // Spawn initial tornadoes at different positions
    println!("{}", "Initializing tornado swarm...".yellow());
    for i in 0..3 {
        let position = Vec3::new(
            (i as f32) * 15.0,
            (i as f32) * 10.0,
            5.0,
        );
        swarm.spawn_tornado(position).await;
    }
    
    // Create prompt processor
    let processor = PromptProcessor::new(swarm.clone());
    
    // Example prompts to process
    let prompts = vec![
        "Transform this text into something amazing",
        "Hello swarm, process my thoughts",
        "Distributed computation through whirlwinds",
    ];
    
    println!();
    println!("{}", "═══════════════════════════════════════════".bright_blue());
    println!("{}", "Processing prompts through the swarm...".bright_green().bold());
    println!("{}", "═══════════════════════════════════════════".bright_blue());
    println!();
    
    for prompt in prompts {
        // Send prompt into the swarm
        let prompt_id = processor.send_prompt(prompt).await;
        
        // Simulate processing time with visual feedback
        for _i in 0..5 {
            processor.process_step(0.1).await;
            print!("{}", ".".bright_yellow());
            use std::io::{self, Write};
            io::stdout().flush().unwrap();
            sleep(Duration::from_millis(200)).await;
        }
        println!();
        
        // Collect results
        if let Some(result) = processor.collect_results(prompt_id).await {
            println!("{}", format!("📊 Result: {}", result).bright_green());
        }
        
        println!("{}", "───────────────────────────────────────────".bright_black());
        println!();
    }
    
    // Show final swarm visualization
    println!();
    println!("{}", "╔══════════════════════════════════════════╗".bright_magenta());
    println!("{}", "║         SWARM VISUALIZATION              ║".bright_magenta().bold());
    println!("{}", "╚══════════════════════════════════════════╝".bright_magenta());
    
    visualize_swarm().await;
    
    println!();
    println!("{}", "✨ Wingbeat swarm processing complete!".bright_green().bold());
}

async fn visualize_swarm() {
    let frames = vec![
        r#"
        🌪️     🌀     🌪️
         \     |     /
          \    |    /
           🧩--🧩--🧩
          /    |    \
         /     |     \
        🌊     🌊     🌊
        "#,
        r#"
        🌀     🌪️     🌀
         |     /\    |
         |    /  \   |
        🧩---🧩  🧩---🧩
         |    \  /   |
         |     \/    |
        🌊     🌊     🌊
        "#,
        r#"
        🌪️     🌀     🌪️
          \   / \   /
           \ /   \ /
            🧩---🧩
           / \   / \
          /   \ /   \
        🌊     🌊     🌊
        "#,
    ];
    
    for frame in frames {
        print!("\x1B[2J\x1B[1;1H"); // Clear screen
        println!("{}", frame.bright_cyan());
        sleep(Duration::from_millis(500)).await;
    }
}

