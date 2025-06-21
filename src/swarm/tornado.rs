use crate::core::subgraph::Subgraph;
use std::collections::HashMap;
use uuid::Uuid;
use tokio::sync::RwLock;
use std::sync::Arc;
use rand::Rng;
use colored::*;

/// Represents a tornado/whirlwind in the swarm
#[derive(Debug)]
pub struct Tornado {
    pub id: Uuid,
    pub eye: Vec3,  // Center of the tornado
    pub radius: f32,
    pub angular_velocity: f32,
    pub height: f32,
    pub subgraphs: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Subgraph>>>>>,
}

/// 3D position for tornado dynamics
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn distance(&self, other: &Vec3) -> f32 {
        ((self.x - other.x).powi(2) + 
         (self.y - other.y).powi(2) + 
         (self.z - other.z).powi(2)).sqrt()
    }
}

impl Tornado {
    pub fn new(position: Vec3) -> Self {
        Self {
            id: Uuid::new_v4(),
            eye: position,
            radius: rand::thread_rng().gen_range(5.0..20.0),
            angular_velocity: rand::thread_rng().gen_range(0.5..2.0),
            height: rand::thread_rng().gen_range(10.0..50.0),
            subgraphs: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Sweep up a subgraph into the tornado
    pub async fn sweep_up(&self, subgraph: Arc<RwLock<Subgraph>>) {
        let mut subgraphs = self.subgraphs.write().await;
        let sg_id = subgraph.read().await.id;
        
        println!("{}", format!("🌪️  Tornado {} sweeping up subgraph {}", 
            self.id.to_string()[..8].cyan(), 
            sg_id.to_string()[..8].yellow()
        ));
        
        subgraphs.insert(sg_id, subgraph);
    }

    /// Spin the tornado, causing subgraphs to interact
    pub async fn spin(&self) {
        let subgraphs = self.subgraphs.read().await;
        
        if subgraphs.len() < 2 {
            return;
        }

        println!("{}", "🌀 Tornado spinning, subgraphs interacting...".bright_blue());
        
        // Randomly select pairs of subgraphs to potentially merge or split
        let ids: Vec<Uuid> = subgraphs.keys().cloned().collect();
        
        for i in 0..ids.len() {
            for j in i+1..ids.len() {
                let sg1 = subgraphs.get(&ids[i]).unwrap();
                let sg2 = subgraphs.get(&ids[j]).unwrap();
                
                let sg1_read = sg1.read().await;
                let sg2_read = sg2.read().await;
                
                if sg1_read.can_connect_with(&sg2_read) {
                    println!("{}", format!("⚡ Subgraphs {} and {} connecting!", 
                        ids[i].to_string()[..8].green(),
                        ids[j].to_string()[..8].green()
                    ));
                }
            }
        }
    }

    /// Release subgraphs from the tornado
    pub async fn release(&self, count: usize) -> Vec<Arc<RwLock<Subgraph>>> {
        let mut subgraphs = self.subgraphs.write().await;
        let mut released = Vec::new();
        
        let keys: Vec<Uuid> = subgraphs.keys().cloned().collect();
        
        for i in 0..count.min(keys.len()) {
            if let Some(sg) = subgraphs.remove(&keys[i]) {
                println!("{}", format!("💨 Releasing subgraph {} from tornado", 
                    keys[i].to_string()[..8].bright_yellow()
                ));
                released.push(sg);
            }
        }
        
        released
    }
}

/// Manages multiple tornadoes in the swarm
#[derive(Debug)]
pub struct TornadoSwarm {
    pub tornadoes: Arc<RwLock<Vec<Tornado>>>,
}

impl TornadoSwarm {
    pub fn new() -> Self {
        Self {
            tornadoes: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Spawn a new tornado at a given position
    pub async fn spawn_tornado(&self, position: Vec3) {
        let tornado = Tornado::new(position);
        println!("{}", format!("🌪️  Spawning new tornado at ({:.1}, {:.1}, {:.1})", 
            position.x, position.y, position.z
        ).bright_cyan());
        
        self.tornadoes.write().await.push(tornado);
    }

    /// Simulate the swarm dynamics
    pub async fn simulate_step(&self, delta_time: f32) {
        let tornadoes = self.tornadoes.read().await;
        
        for tornado in tornadoes.iter() {
            // Move tornado
            let mut rng = rand::thread_rng();
            let _new_position = Vec3::new(
                tornado.eye.x + rng.gen_range(-1.0..1.0) * delta_time,
                tornado.eye.y + rng.gen_range(-1.0..1.0) * delta_time,
                tornado.eye.z,
            );
            
            // Spin tornado
            tornado.spin().await;
        }
    }
} 