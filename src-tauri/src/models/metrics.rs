use std::time::Instant;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub disk_usage: f32,
    pub download_speed: f64,
    pub upload_speed: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskMetric {
    pub name: String,
    pub mount_point: String,
    pub total_bytes: u64,
    pub available_bytes: u64,
    pub used_bytes: u64,
    pub used_percent: f32,
}

#[derive(Debug, Clone)]
pub struct DiskState {
    pub last_read_bytes: u64,
    pub last_written_bytes: u64,
    pub last_update: Option<Instant>,
}

impl DiskState {
    pub fn default() -> Self {
        Self {
            last_read_bytes: 0,
            last_written_bytes: 0,
            last_update: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NetworkState {
    pub last_received: u64,
    pub last_transmitted: u64,
    pub last_update: Option<Instant>,
}

impl NetworkState {
    pub fn default() -> Self {
        NetworkState {
            last_received: 0,
            last_transmitted: 0,
            last_update: None,
        }
    }
}
