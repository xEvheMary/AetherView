use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub process_count: usize,
    pub cpu_usage: f32,
    pub memory_mb: u64,
}