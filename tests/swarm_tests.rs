use wingbeat::{Subgraph, TornadoSwarm, Vec3, PromptProcessor};
use std::sync::Arc;

#[tokio::test]
async fn test_subgraph_creation() {
    let subgraph = Subgraph::new();
    assert!(subgraph.parent.is_none());
    assert!(subgraph.children.is_empty());
    assert!(subgraph.tornado_strength >= 0.0 && subgraph.tornado_strength <= 1.0);
}

#[tokio::test]
async fn test_subgraph_splitting() {
    let mut parent = Subgraph::new();
    let parent_id = parent.id;
    
    let children = parent.split(3).await;
    
    assert_eq!(children.len(), 3);
    assert_eq!(parent.children.len(), 3);
    
    for child in &children {
        assert_eq!(child.parent, Some(parent_id));
    }
}

#[tokio::test]
async fn test_subgraph_connectivity() {
    let mut sg1 = Subgraph::new();
    let mut sg2 = Subgraph::new();
    
    // Set similar strengths - should connect
    sg1.tornado_strength = 0.5;
    sg2.tornado_strength = 0.6;
    assert!(sg1.can_connect_with(&sg2));
    
    // Set different strengths - should not connect
    sg1.tornado_strength = 0.1;
    sg2.tornado_strength = 0.9;
    assert!(!sg1.can_connect_with(&sg2));
}

#[tokio::test]
async fn test_tornado_creation() {
    let swarm = TornadoSwarm::new();
    assert!(swarm.tornadoes.read().await.is_empty());
    
    swarm.spawn_tornado(Vec3::new(0.0, 0.0, 0.0)).await;
    assert_eq!(swarm.tornadoes.read().await.len(), 1);
}

#[tokio::test]
async fn test_prompt_processing() {
    let swarm = Arc::new(TornadoSwarm::new());
    let processor = PromptProcessor::new(swarm);
    
    let prompt_id = processor.send_prompt("Test prompt").await;
    
    // Process a few steps
    for _ in 0..3 {
        processor.process_step(0.1).await;
    }
    
    let result = processor.collect_results(prompt_id).await;
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "TEST PROMPT");
}

#[tokio::test]
async fn test_vec3_distance() {
    let v1 = Vec3::new(0.0, 0.0, 0.0);
    let v2 = Vec3::new(3.0, 4.0, 0.0);
    
    assert_eq!(v1.distance(&v2), 5.0);
} 