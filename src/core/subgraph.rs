use std::collections::HashMap;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use petgraph::graph::Graph;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Represents a single computation node in a subgraph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeNode {
    pub id: Uuid,
    pub operation: Operation,
    pub state: NodeState,
    pub metadata: HashMap<String, String>,
}

/// Types of operations a node can perform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operation {
    Transform(String),
    Split,
    Merge,
    Process(String),
    Filter(String),
    Aggregate,
}

/// State of a computation node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeState {
    Idle,
    Processing,
    Splitting,
    Merging,
    Complete,
}

/// A subgraph that can split, merge, and reorganize
#[derive(Debug)]
pub struct Subgraph {
    pub id: Uuid,
    pub graph: Arc<RwLock<Graph<ComputeNode, f32>>>,
    pub parent: Option<Uuid>,
    pub children: Vec<Uuid>,
    pub tornado_strength: f32, // How strongly it's caught in the whirlwind
}

impl Subgraph {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            graph: Arc::new(RwLock::new(Graph::new())),
            parent: None,
            children: Vec::new(),
            tornado_strength: rand::random::<f32>(),
        }
    }

    /// Split this subgraph into multiple smaller subgraphs
    pub async fn split(&mut self, num_splits: usize) -> Vec<Subgraph> {
        let mut splits = Vec::new();
        
        for _ in 0..num_splits {
            let mut child = Subgraph::new();
            child.parent = Some(self.id);
            self.children.push(child.id);
            splits.push(child);
        }
        
        splits
    }

    /// Merge with another subgraph
    pub async fn merge(&mut self, other: Subgraph) -> Result<(), String> {
        let other_graph = other.graph.read().await;
        let mut self_graph = self.graph.write().await;
        
        // Merge the graphs
        for node in other_graph.node_weights() {
            self_graph.add_node(node.clone());
        }
        
        // Update tornado strength as average
        self.tornado_strength = (self.tornado_strength + other.tornado_strength) / 2.0;
        
        Ok(())
    }

    /// Check if this subgraph can connect with another (like legos)
    pub fn can_connect_with(&self, other: &Subgraph) -> bool {
        // Subgraphs can connect if their tornado strengths are compatible
        (self.tornado_strength - other.tornado_strength).abs() < 0.3
    }
}

impl Clone for Subgraph {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            graph: Arc::clone(&self.graph),
            parent: self.parent,
            children: self.children.clone(),
            tornado_strength: self.tornado_strength,
        }
    }
} 