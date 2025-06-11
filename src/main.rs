mod orchestrator;

use orchestrator::{Node, Orchestrator, TaskMessage};
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Example usage of the orchestrator
    let orchestrator = Orchestrator::new();

    let node = Node {
        id: "node-1".into(),
        endpoint: "http://localhost:8080/task".into(),
    };

    let msg = TaskMessage {
        task_id: "demo".into(),
        payload: json!({"example": true}),
    };

    // Ignoring errors for this demo
    let _ = orchestrator.dispatch_task(&node, &msg).await;

    Ok(())
}
