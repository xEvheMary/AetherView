use crate::models::metrics::{NetworkState, SystemMetrics, DiskMetric, DiskState};
use crate::services::metric_service::{collect_system_metrics, collect_disk_metrics};
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub fn get_system_metrics(network_state: State<Mutex<NetworkState>>, disk_state: State<Mutex<DiskState>>) -> SystemMetrics {
    let mut net_state = network_state.lock().unwrap();
    let mut dsk_state = disk_state.lock().unwrap();
    collect_system_metrics(&mut net_state, &mut dsk_state)
}



#[tauri::command]
pub fn get_disk_metrics() -> Vec<DiskMetric> {
    collect_disk_metrics()
}
