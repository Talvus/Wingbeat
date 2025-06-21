use crate::core::subgraph::{Subgraph, ComputeNode, Operation, NodeState};
use crate::swarm::tornado::{TornadoSwarm, Vec3};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use colored::*;
use std::collections::HashMap;

/// Represents a prompt being processed through the swarm
#[derive(Debug, Clone)]
pub struct SwarmPrompt {
    pub id: Uuid,
    pub content: String,
    pub origin: Vec3,
    pub status: PromptStatus,
    pub fragments: Vec<PromptFragment>,
}

#[derive(Debug, Clone)]
pub enum PromptStatus {
    Sent,
    InWhirlwind,
    Processing,
    Assembling,
    Complete,
}

#[derive(Debug, Clone)]
pub struct PromptFragment {
    pub id: Uuid,
    pub content: String,
    pub subgraph_id: Uuid,
    pub processed: bool,
}

/// Main prompt processor that orchestrates the swarm
pub struct PromptProcessor {
    pub swarm: Arc<TornadoSwarm>,
    pub active_prompts: Arc<RwLock<HashMap<Uuid, SwarmPrompt>>>,
}

impl PromptProcessor {
    pub fn new(swarm: Arc<TornadoSwarm>) -> Self {
        Self {
            swarm,
            active_prompts: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Send a prompt into the swarm
    pub async fn send_prompt(&self, prompt: &str) -> Uuid {
        let prompt_id = Uuid::new_v4();
        
        println!("{}", format!("📤 Sending prompt into the swarm: '{}'", prompt).bright_green());
        
        // Create prompt and fragment it
        let swarm_prompt = SwarmPrompt {
            id: prompt_id,
            content: prompt.to_string(),
            origin: Vec3::new(0.0, 0.0, 0.0),
            status: PromptStatus::Sent,
            fragments: self.fragment_prompt(prompt, prompt_id),
        };
        
        // Store active prompt
        self.active_prompts.write().await.insert(prompt_id, swarm_prompt.clone());
        
        // Distribute fragments to tornadoes
        self.distribute_fragments(swarm_prompt).await;
        
        prompt_id
    }

    /// Fragment a prompt into smaller pieces
    fn fragment_prompt(&self, prompt: &str, _prompt_id: Uuid) -> Vec<PromptFragment> {
        let words: Vec<&str> = prompt.split_whitespace().collect();
        let mut fragments = Vec::new();
        
        // Create fragments of varying sizes (like irregular lego pieces)
        let mut i = 0;
        while i < words.len() {
            let fragment_size = rand::random::<usize>() % 3 + 1; // 1-3 words per fragment
            let end = (i + fragment_size).min(words.len());
            
            let fragment_content = words[i..end].join(" ");
            
            fragments.push(PromptFragment {
                id: Uuid::new_v4(),
                content: fragment_content,
                subgraph_id: Uuid::new_v4(),
                processed: false,
            });
            
            i = end;
        }
        
        println!("{}", format!("🧩 Fragmented prompt into {} pieces", fragments.len()).yellow());
        fragments
    }

    /// Distribute prompt fragments across the tornado swarm
    async fn distribute_fragments(&self, prompt: SwarmPrompt) {
        let tornadoes = self.swarm.tornadoes.read().await;
        
        if tornadoes.is_empty() {
            // Spawn tornadoes if none exist
            drop(tornadoes);
            for i in 0..3 {
                let pos = Vec3::new(
                    (i as f32) * 10.0,
                    (i as f32) * 5.0,
                    0.0
                );
                self.swarm.spawn_tornado(pos).await;
            }
        }
        
        let tornadoes = self.swarm.tornadoes.read().await;
        
        // Create subgraphs for each fragment and sweep them into tornadoes
        for (i, fragment) in prompt.fragments.iter().enumerate() {
            let subgraph = Subgraph::new();
            
            // Add compute node to process this fragment
            let node = ComputeNode {
                id: Uuid::new_v4(),
                operation: Operation::Process(fragment.content.clone()),
                state: NodeState::Idle,
                metadata: HashMap::new(),
            };
            
            subgraph.graph.write().await.add_node(node);
            
            // Select a tornado to sweep this subgraph
            let tornado_idx = i % tornadoes.len();
            let tornado = &tornadoes[tornado_idx];
            
            tornado.sweep_up(Arc::new(RwLock::new(subgraph))).await;
        }
        
        // Update prompt status
        self.active_prompts.write().await.get_mut(&prompt.id)
            .map(|p| p.status = PromptStatus::InWhirlwind);
        
        println!("{}", "🌪️  Fragments swept up into the whirlwind!".bright_cyan());
    }

    /// Process the swarm for one time step
    pub async fn process_step(&self, delta_time: f32) {
        // Simulate swarm dynamics
        self.swarm.simulate_step(delta_time).await;
        
        // Check for completed processing
        let tornadoes = self.swarm.tornadoes.read().await;
        
        for tornado in tornadoes.iter() {
            tornado.spin().await;
        }
    }

    /// Collect results from the swarm
    pub async fn collect_results(&self, prompt_id: Uuid) -> Option<String> {
        let mut prompts = self.active_prompts.write().await;
        
        if let Some(prompt) = prompts.get_mut(&prompt_id) {
            prompt.status = PromptStatus::Assembling;
            
            println!("{}", "📥 Collecting results from the swarm...".bright_magenta());
            
            // Simulate result assembly
            let processed_content = prompt.content.to_uppercase();
            
            prompt.status = PromptStatus::Complete;
            
            println!("{}", format!("✅ Results assembled: {}", processed_content).bright_green());
            
            Some(processed_content)
        } else {
            None
        }
    }
} 