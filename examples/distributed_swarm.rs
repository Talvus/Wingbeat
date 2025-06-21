use wingbeat::{TornadoSwarm, PromptProcessor, Vec3, Subgraph};
use std::sync::Arc;
use colored::*;
use tokio::time::{sleep, Duration};
use tokio::sync::RwLock;

/// Example demonstrating distributed swarm behavior
#[tokio::main]
async fn main() {
    println!("{}", "═══════════════════════════════════════════".bright_cyan());
    println!("{}", "    DISTRIBUTED SWARM DEMONSTRATION".bright_cyan().bold());
    println!("{}", "═══════════════════════════════════════════".bright_cyan());
    println!();
    
    // Create a swarm with multiple tornado regions
    let swarm = Arc::new(TornadoSwarm::new());
    
    // Spawn tornadoes in different regions (simulating distributed nodes)
    let regions = vec![
        ("North Region", Vec3::new(0.0, 50.0, 10.0)),
        ("East Region", Vec3::new(50.0, 0.0, 10.0)),
        ("South Region", Vec3::new(0.0, -50.0, 10.0)),
        ("West Region", Vec3::new(-50.0, 0.0, 10.0)),
        ("Central Hub", Vec3::new(0.0, 0.0, 20.0)),
    ];
    
    for (name, position) in regions {
        println!("{}", format!("🌍 Spawning tornado in {}", name).bright_blue());
        swarm.spawn_tornado(position).await;
        sleep(Duration::from_millis(100)).await;
    }
    
    println!();
    
    // Create subgraphs that will interact
    let mut subgraphs = Vec::new();
    
    for i in 0..10 {
        let subgraph = Arc::new(RwLock::new(Subgraph::new()));
        
        // Set varying tornado strengths
        subgraph.write().await.tornado_strength = (i as f32) * 0.1;
        
        subgraphs.push(subgraph);
    }
    
    // Demonstrate subgraph interactions
    println!("{}", "🔄 Demonstrating subgraph interactions...".yellow());
    println!();
    
    for i in 0..subgraphs.len() {
        for j in i+1..subgraphs.len() {
            let sg1 = subgraphs[i].read().await;
            let sg2 = subgraphs[j].read().await;
            
            if sg1.can_connect_with(&sg2) {
                println!("{}", 
                    format!("✅ Subgraph {} can connect with Subgraph {} (strengths: {:.2} ~ {:.2})",
                        i, j, sg1.tornado_strength, sg2.tornado_strength
                    ).green()
                );
            } else {
                println!("{}", 
                    format!("❌ Subgraph {} cannot connect with Subgraph {} (strengths: {:.2} ≠ {:.2})",
                        i, j, sg1.tornado_strength, sg2.tornado_strength
                    ).red()
                );
            }
        }
    }
    
    println!();
    
    // Demonstrate splitting
    println!("{}", "🧩 Demonstrating subgraph splitting...".yellow());
    let mut parent = Subgraph::new();
    let children = parent.split(3).await;
    
    println!("{}", format!("Parent subgraph {} split into {} children", 
        parent.id.to_string()[..8].bright_cyan(),
        children.len()
    ));
    
    for (i, child) in children.iter().enumerate() {
        println!("{}", format!("  └─ Child {}: {}", 
            i + 1, 
            child.id.to_string()[..8].bright_green()
        ));
    }
    
    println!();
    
    // Create processor and send a complex prompt
    let processor = PromptProcessor::new(swarm.clone());
    
    let complex_prompt = "The quick brown fox jumps over the lazy dog while thinking about distributed computation in tornado-like swarms";
    
    println!("{}", "📝 Sending complex prompt through distributed swarm...".bright_magenta());
    let prompt_id = processor.send_prompt(complex_prompt).await;
    
    // Simulate distributed processing with status updates
    for step in 0..10 {
        processor.process_step(0.2).await;
        
        let progress = "█".repeat(step + 1) + &"░".repeat(9 - step);
        print!("\r{} Processing: [{}] {}%", 
            "⚡".yellow(), 
            progress.bright_blue(), 
            (step + 1) * 10
        );
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
        
        sleep(Duration::from_millis(300)).await;
    }
    println!();
    
    // Collect and display results
    if let Some(result) = processor.collect_results(prompt_id).await {
        println!();
        println!("{}", "📊 Distributed Processing Complete!".bright_green().bold());
        println!("{}", format!("Original: {}", complex_prompt).dimmed());
        println!("{}", format!("Result:   {}", result).bright_cyan());
    }
    
    println!();
    println!("{}", "✨ Distributed swarm demonstration complete!".bright_green().bold());
} 