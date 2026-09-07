use tauri::State;
use std::sync::Mutex;
use crate::models::metrics::{SystemMetrics, NetworkState};
use crate::services::metric_service::collect_system_metrics;

#[tauri::command]
pub fn get_system_metrics(
    network_state: State<Mutex<NetworkState>>,
) -> SystemMetrics {
    let mut state = network_state.lock().unwrap();
    collect_system_metrics(&mut state)
}