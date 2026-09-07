use sysinfo::System;
use crate::models::metrics::{SystemMetrics, NetworkState};
use crate::services::process_service::get_network_speed;

pub fn collect_system_metrics(state: &mut NetworkState) -> SystemMetrics {
    let mut system = System::new_all();
    system.refresh_cpu_all();
    system.refresh_memory();
    let cpu_usage = system.global_cpu_usage();
    let ram_usage_percent = (system.used_memory() as f32 / system.total_memory() as f32) * 100.0;
    let (download, upload) = get_network_speed(state);
    SystemMetrics {
        cpu_usage: cpu_usage,
        memory_usage: ram_usage_percent,
        disk_usage: 0.0,
        download_speed: download,
        upload_speed: upload,
    }
}