use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorTarget {
    pub id: String,
    pub name: String,
    pub method: String,
    pub endpoint: String,
    pub interval_seconds: u64,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorStatus {
    pub id: String,
    pub healthy: bool,
    pub response_time_ms: Option<u64>,
    pub last_checked: Option<u64>,
    pub last_error: Option<String>,
    pub history: Vec<Option<u64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonitorState {
    pub targets: Vec<MonitorTarget>,
    pub statuses: HashMap<String, MonitorStatus>,
}

impl MonitorState {
    pub fn default() -> Self {
        MonitorState {
            targets: Vec::new(),
            statuses: HashMap::new(),
        }
    }

    pub fn update_status(&mut self, status: MonitorStatus) {
        self.statuses.insert(status.id.clone(), status);
    }

    pub fn get_status(&self, id: &str) -> Option<&MonitorStatus> {
        self.statuses.get(id)
    }
}
