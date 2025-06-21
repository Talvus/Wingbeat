# Wingbeat 🌪️

A swarm-based distributed computation protocol where subgraphs of computation graphs organize like tornadoes and whirlwinds, splitting, blending, and forging connections like legos in a dynamic storm.

## Vision

Wingbeat reimagines distributed computation as a natural phenomenon - where computational subgraphs behave like tornadoes in a swarm:

- **🌪️ Tornadoes** sweep up subgraphs and spin them around
- **🧩 Subgraphs** split, merge, and connect like legos
- **📤 Prompts** are fragmented and swept into the whirlwind
- **📥 Results** emerge from the swarm, transformed and complete

## Features

- **Dynamic Subgraph Management**: Subgraphs can split into children, merge with others, and check compatibility
- **Tornado Swarm Dynamics**: Multiple tornadoes move through 3D space, sweeping up and releasing subgraphs
- **Prompt Processing Pipeline**: Prompts are fragmented and distributed across the swarm for processing
- **Visual Feedback**: Rich terminal output with emojis and colors showing the swarm in action
- **Async Architecture**: Built on Tokio for high-performance concurrent operations

## Quick Start

```bash
# Clone the repository
git clone https://github.com/yourusername/wingbeat
cd wingbeat

# Build the project
cargo build

# Run the main demo
cargo run

# Run examples
cargo run --example distributed_swarm
cargo run --example visual_demo

# Run tests
cargo test

# Run benchmarks
cargo bench
```

## Architecture

### Core Components

- **Subgraph** (`src/core/subgraph.rs`)
  - Represents a computation unit with nodes and edges
  - Can split, merge, and check connectivity with other subgraphs
  - Has a "tornado strength" that determines compatibility

- **Tornado** (`src/swarm/tornado.rs`)
  - Sweeps up subgraphs and spins them
  - Causes subgraphs to interact and potentially connect
  - Moves through 3D space with dynamic behavior

- **PromptProcessor** (`src/computation/prompt_processor.rs`)
  - Fragments prompts into pieces
  - Distributes fragments across the tornado swarm
  - Collects and assembles results

### Example Usage

```rust
use wingbeat::{TornadoSwarm, PromptProcessor, Vec3};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Create a tornado swarm
    let swarm = Arc::new(TornadoSwarm::new());
    
    // Spawn tornadoes at different positions
    swarm.spawn_tornado(Vec3::new(0.0, 0.0, 5.0)).await;
    swarm.spawn_tornado(Vec3::new(15.0, 10.0, 5.0)).await;
    
    // Create prompt processor
    let processor = PromptProcessor::new(swarm);
    
    // Send a prompt into the swarm
    let prompt_id = processor.send_prompt("Hello, swarm!").await;
    
    // Process the swarm
    for _ in 0..5 {
        processor.process_step(0.1).await;
    }
    
    // Collect results
    if let Some(result) = processor.collect_results(prompt_id).await {
        println!("Result: {}", result);
    }
}
```

## Visual Output

When you run Wingbeat, you'll see dynamic visualizations like:

```
🌪️  Spawning new tornado at (0.0, 50.0, 10.0)
🧩 Fragmented prompt into 3 pieces
🌪️  Tornado 5135dd98 sweeping up subgraph a5ff81de
🌀 Tornado spinning, subgraphs interacting...
⚡ Subgraphs efd17953 and e0f79202 connecting!
📥 Collecting results from the swarm...
✅ Results assembled: HELLO WORLD

        🌪️     🌀     🌪️
         \     |     /
          \    |    /
           🧩--🧩--🧩
          /    |    \
         /     |     \
        🌊     🌊     🌊
```

## Future Development

The current implementation provides a foundation for:

- **Distributed Nodes**: Running tornadoes on different machines
- **Real Computation**: Implementing actual transformations in compute nodes
- **ML Integration**: Using the swarm for distributed model inference
- **Persistence**: Saving and loading swarm states
- **Network Protocol**: Communication between distributed tornado instances

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues.

## License

MIT License - see LICENSE file for details.
