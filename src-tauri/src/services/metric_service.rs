use std::time::Instant;

use crate::models::metrics::{DiskMetric, NetworkState, DiskState, SystemMetrics};
use crate::services::process_service::get_network_speed;
use sysinfo::{Disks, System};

pub fn collect_system_metrics(state: &mut NetworkState, disk_state: &mut DiskState) -> SystemMetrics {
    let mut system = System::new_all();
    system.refresh_cpu_all();
    system.refresh_memory();
    let cpu_usage = system.global_cpu_usage();
    let ram_usage_percent = (system.used_memory() as f32 / system.total_memory() as f32) * 100.0;
    // 
    let disk_usage_percent = get_disk_activity(disk_state);
    // Network speed
    let (download, upload) = get_network_speed(state);
    SystemMetrics {
        cpu_usage: cpu_usage,
        memory_usage: ram_usage_percent,
        disk_usage: disk_usage_percent,
        download_speed: download,
        upload_speed: upload,
    }
}

pub fn get_disk_activity(state: &mut DiskState) -> f32 {
    let now = Instant::now();
    let elapsed = match state.last_update {
        Some(prev) => now.duration_since(prev).as_secs_f64(),
        None => 0.0,
    };

    let disks = Disks::new_with_refreshed_list();

    let total_read: u64 = disks.list().iter().map(|d| d.usage().total_read_bytes).sum();
    let total_written: u64 = disks.list().iter().map(|d| d.usage().total_written_bytes).sum();

    let percent = if elapsed > 0.0 {
        let read_delta = total_read.saturating_sub(state.last_read_bytes) as f64;
        let write_delta = total_written.saturating_sub(state.last_written_bytes) as f64;
        let bytes_per_sec = (read_delta + write_delta) / elapsed;

        // Tunable normalization factor for "activity-like %" feel.
        // 100 MB/s => 100%
        let normalized = (bytes_per_sec / 100_000_000.0) * 100.0;
        normalized.clamp(0.0, 100.0) as f32
    } else {
        0.0
    };

    state.last_read_bytes = total_read;
    state.last_written_bytes = total_written;
    state.last_update = Some(now);

    percent
}

pub fn collect_disk_metrics() -> Vec<DiskMetric> {
    let disks = Disks::new_with_refreshed_list();

    disks.list().iter().map(|disk| {
        let total = disk.total_space();
        let available = disk.available_space();
        let used = total.saturating_sub(available);
        let used_percent = if total > 0 {(used as f64 / total as f64 * 100.0) as f32} else {0.0};
        DiskMetric {
            name: disk.name().to_string_lossy().to_string(),
            mount_point: disk.mount_point().to_string_lossy().to_string(),
            total_bytes: total,
            available_bytes: available,
            used_bytes: used,
            used_percent,
        }
    })
    .collect()
}
