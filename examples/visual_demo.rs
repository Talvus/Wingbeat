use wingbeat::{TornadoSwarm, Vec3, Subgraph};
use std::sync::Arc;
use colored::*;
use tokio::time::{sleep, Duration};
use tokio::sync::RwLock;

/// Visual demonstration of tornado swarm dynamics
#[tokio::main]
async fn main() {
    println!("{}", "╔══════════════════════════════════════════╗".bright_magenta());
    println!("{}", "║      TORNADO SWARM VISUALIZATION         ║".bright_magenta().bold());
    println!("{}", "╚══════════════════════════════════════════╝".bright_magenta());
    println!();
    
    // Create the swarm
    let swarm = Arc::new(TornadoSwarm::new());
    
    // Spawn tornadoes in a pattern
    let positions = vec![
        Vec3::new(-20.0, -20.0, 0.0),
        Vec3::new(20.0, -20.0, 0.0),
        Vec3::new(0.0, 20.0, 0.0),
    ];
    
    for (i, pos) in positions.iter().enumerate() {
        println!("{}", format!("🌪️  Spawning tornado {} at ({:.1}, {:.1}, {:.1})", 
            i + 1, pos.x, pos.y, pos.z
        ).bright_cyan());
        swarm.spawn_tornado(*pos).await;
    }
    
    println!();
    println!("{}", "Creating subgraphs and releasing them into the swarm...".yellow());
    
    // Create and release subgraphs
    let tornadoes = swarm.tornadoes.read().await;
    
    for (i, tornado) in tornadoes.iter().enumerate() {
        for j in 0..3 {
            let subgraph = Arc::new(RwLock::new(Subgraph::new()));
            
            // Set properties
            {
                let mut sg = subgraph.write().await;
                sg.tornado_strength = ((i + j) as f32) * 0.15;
            }
            
            tornado.sweep_up(subgraph).await;
            sleep(Duration::from_millis(100)).await;
        }
    }
    
    drop(tornadoes);
    
    println!();
    println!("{}", "Starting swarm simulation...".bright_green().bold());
    println!("{}", "Watch as tornadoes move and subgraphs interact!".dimmed());
    println!();
    
    // Run simulation for several steps
    for step in 0..10 {
        println!("{}", format!("═══ Simulation Step {} ═══", step + 1).bright_blue());
        
        // Simulate movement
        swarm.simulate_step(1.0).await;
        
        // Show tornado positions
        let tornadoes = swarm.tornadoes.read().await;
        for (i, tornado) in tornadoes.iter().enumerate() {
            let subgraph_count = tornado.subgraphs.read().await.len();
            
            println!("{}", format!(
                "  Tornado {}: Position ({:.1}, {:.1}, {:.1}) | {} subgraphs",
                i + 1, 
                tornado.eye.x, 
                tornado.eye.y, 
                tornado.eye.z,
                subgraph_count
            ).cyan());
        }
        
        // Occasionally release and sweep up subgraphs
        if step % 3 == 0 && step > 0 {
            println!("{}", "  💨 Releasing and redistributing subgraphs...".yellow());
            
            for tornado in tornadoes.iter() {
                let released = tornado.release(1).await;
                
                // Re-sweep into different tornado
                if !released.is_empty() {
                    let target_idx = rand::random::<usize>() % tornadoes.len();
                    tornadoes[target_idx].sweep_up(released[0].clone()).await;
                }
            }
        }
        
        drop(tornadoes);
        
        // Visual representation
        print_swarm_visualization(step).await;
        
        sleep(Duration::from_millis(500)).await;
        println!();
    }
    
    println!("{}", "✨ Swarm simulation complete!".bright_green().bold());
    println!();
    
    // Show final statistics
    let tornadoes = swarm.tornadoes.read().await;
    let total_subgraphs: usize = futures::future::join_all(
        tornadoes.iter().map(|t| async {
            t.subgraphs.read().await.len()
        })
    ).await.iter().sum();
    
    println!("{}", "📊 Final Statistics:".bright_yellow());
    println!("{}", format!("  • Total Tornadoes: {}", tornadoes.len()));
    println!("{}", format!("  • Total Subgraphs: {}", total_subgraphs));
    println!("{}", format!("  • Average Subgraphs per Tornado: {:.1}", 
        total_subgraphs as f32 / tornadoes.len() as f32
    ));
}

async fn print_swarm_visualization(step: usize) {
    let frames = vec![
        vec![
            "     🌪️           🌀           🌪️     ",
            "    / \\          / \\          / \\    ",
            "   🧩 🧩        🧩 🧩        🧩 🧩   ",
            "    \\ /          \\ /          \\ /    ",
            "     🌊           🌊           🌊     ",
        ],
        vec![
            "      🌀         🌪️         🌀      ",
            "     / \\        /|\\        / \\     ",
            "    🧩🧩🧩    🧩 🧩 🧩    🧩🧩🧩    ",
            "     \\ /        \\|/        \\ /     ",
            "      🌊         🌊         🌊      ",
        ],
        vec![
            "    🌪️     🌀     🌪️     🌀    ",
            "    |\\     /|     |\\     /|    ",
            "   🧩 🧩 🧩 🧩 🧩 🧩 🧩 🧩   ",
            "    |/     \\|     |/     \\|    ",
            "    🌊     🌊     🌊     🌊    ",
        ],
    ];
    
    let frame = &frames[step % frames.len()];
    
    println!();
    for line in frame {
        println!("{}", line.bright_cyan());
    }
} 