use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Represents a remote node in the Wingbeat network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub endpoint: String,
}

/// Basic message that can be sent between nodes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskMessage {
    pub task_id: String,
    pub payload: serde_json::Value,
}

/// Orchestrator handles communication with nodes.
#[derive(Debug, Default)]
pub struct Orchestrator {
    client: Client,
}

impl Orchestrator {
    /// Create a new orchestrator.
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Send a task to a remote node asynchronously.
    pub async fn dispatch_task(&self, node: &Node, msg: &TaskMessage) -> Result<()> {
        self.client
            .post(&node.endpoint)
            .json(msg)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }
}
