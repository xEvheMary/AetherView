use std::{collections::HashMap, time::Instant};

use crate::models::metrics::NetworkState;
use crate::models::process_info::ProcessInfo;
use sysinfo::{Networks, System};

pub fn get_top_processes(limit: usize) -> Vec<ProcessInfo> {
    let mut system = System::new_all();
    system.refresh_all();
    let mut grouped: HashMap<String, ProcessInfo> = HashMap::new();
    for process in system.processes().values() {
        let name = process.name();
        grouped
            .entry(name.to_string_lossy().to_string())
            .and_modify(|entry| {
                entry.memory_mb += process.memory() / 1024 / 1024;
                entry.cpu_usage += process.cpu_usage();
                entry.process_count += 1;
            })
            .or_insert(ProcessInfo {
                pid: process.pid().as_u32(),
                name: name.to_string_lossy().to_string(),
                cpu_usage: process.cpu_usage(),
                memory_mb: process.memory() / 1024 / 1024,
                process_count: 1,
            });
    }
    let mut processes: Vec<ProcessInfo> = grouped.into_values().collect();

    processes.sort_by(|a, b| b.memory_mb.cmp(&a.memory_mb));
    processes.truncate(limit);
    processes
}

pub fn get_network_speed(state: &mut NetworkState) -> (f64, f64) {
    let mut networks = Networks::new_with_refreshed_list();
    networks.refresh(true);
    let total_received: u64 = networks
        .values()
        .map(|network| network.total_received())
        .sum();
    let total_transmitted: u64 = networks
        .values()
        .map(|network| network.total_transmitted())
        .sum();
    let now = Instant::now();
    let (download_speed, upload_speed) = if let Some(last_update) = state.last_update {
        let elapsed = now.duration_since(last_update).as_secs_f64();

        if elapsed > 0.0 {
            let received = total_received.saturating_sub(state.last_received);
            let transmitted = total_transmitted.saturating_sub(state.last_transmitted);

            (received as f64 / elapsed, transmitted as f64 / elapsed)
        } else {
            (0.0, 0.0)
        }
    } else {
        // First call has no previous measurement.
        (0.0, 0.0)
    };
    state.last_received = total_received;
    state.last_transmitted = total_transmitted;
    state.last_update = Some(now);

    (download_speed, upload_speed)
}
