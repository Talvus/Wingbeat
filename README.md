# Wingbeat

Wingbeat aims to orchestrate distributed computation across a network of nodes. 
Subgraphs of a computation graph are delegated to remote nodes and the results 
are gathered to complete the overall computation. This repository currently 
contains a minimal Rust implementation that demonstrates the initial 
networking layer.

## Building

```bash
cargo build
```

## Running the Example

The current binary sends a demo task to a hypothetical node using HTTP. It 
serves as the first step toward a decentralized orchestration layer.

```bash
cargo run
```
