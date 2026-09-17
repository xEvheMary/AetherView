use crate::models::metrics::{NetworkState, SystemMetrics};
use crate::services::metric_service::collect_system_metrics;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub fn get_system_metrics(network_state: State<Mutex<NetworkState>>) -> SystemMetrics {
    let mut state = network_state.lock().unwrap();
    collect_system_metrics(&mut state)
}
