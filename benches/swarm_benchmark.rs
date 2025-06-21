use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wingbeat::{Subgraph, TornadoSwarm, Vec3, PromptProcessor};
use std::sync::Arc;
use tokio::runtime::Runtime;

fn benchmark_subgraph_creation(c: &mut Criterion) {
    c.bench_function("subgraph creation", |b| {
        b.iter(|| {
            let subgraph = Subgraph::new();
            black_box(subgraph);
        });
    });
}

fn benchmark_subgraph_splitting(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("subgraph splitting", |b| {
        b.iter(|| {
            rt.block_on(async {
                let mut subgraph = Subgraph::new();
                let splits = subgraph.split(5).await;
                black_box(splits);
            });
        });
    });
}

fn benchmark_tornado_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("tornado sweep and release", |b| {
        b.iter(|| {
            rt.block_on(async {
                let tornado = wingbeat::swarm::tornado::Tornado::new(Vec3::new(0.0, 0.0, 0.0));
                
                // Sweep up subgraphs
                for _ in 0..10 {
                    let subgraph = Arc::new(tokio::sync::RwLock::new(Subgraph::new()));
                    tornado.sweep_up(subgraph).await;
                }
                
                // Release some
                let released = tornado.release(5).await;
                black_box(released);
            });
        });
    });
}

fn benchmark_prompt_processing(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("prompt processing", |b| {
        b.iter(|| {
            rt.block_on(async {
                let swarm = Arc::new(TornadoSwarm::new());
                let processor = PromptProcessor::new(swarm);
                
                let prompt_id = processor.send_prompt("Test prompt for benchmarking").await;
                
                // Process a few steps
                for _ in 0..5 {
                    processor.process_step(0.1).await;
                }
                
                let result = processor.collect_results(prompt_id).await;
                black_box(result);
            });
        });
    });
}

criterion_group!(
    benches,
    benchmark_subgraph_creation,
    benchmark_subgraph_splitting,
    benchmark_tornado_operations,
    benchmark_prompt_processing
);
criterion_main!(benches); 