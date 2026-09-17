use std::fs;
use std::sync::Mutex;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use tauri::{Manager, State, Emitter};

use crate::utils::data::get_config_path;
use crate::models::monitor::{MonitorState, MonitorStatus, MonitorTarget};
use crate::commands::settings::{load_settings};

#[tauri::command]
pub fn get_monitor_targets(
    state: State<Mutex<MonitorState>>,
) -> Result<Vec<MonitorTarget>, String> {
    Ok(state.lock().unwrap().targets.clone())
}

#[tauri::command]
pub fn add_monitor_targets(
    app: tauri::AppHandle,
    target: MonitorTarget,
) -> Result<(), String> {
    let state = app.state::<Mutex<MonitorState>>();

    let mut monitor_state = state
        .lock()
        .map_err(|e| e.to_string())?;
    let mut target = target;
    if target.id.trim().is_empty() {
        target.id = uuid::Uuid::new_v4().to_string();
    }
    monitor_state.targets.push(target);
    save_monitors(&app, &monitor_state.targets)?;
    Ok(())
}

#[tauri::command]
pub fn get_monitor_state(
    state: State<Mutex<MonitorState>>,
) -> Result<MonitorState, String> {
    Ok(state.lock().unwrap().clone())
}

pub fn load_monitor(app: &tauri::AppHandle) -> Result<(), String> {
    // Load the targets
    let target_path = get_config_path(app)?.join("monitors.json");
    let targets = if !target_path.exists() {
        let monitor_targets: Vec<MonitorTarget> = Vec::new();

        fs::create_dir_all(
            target_path.parent().unwrap()
        ).map_err(|e| e.to_string())?;

        let json = serde_json::to_string_pretty(&monitor_targets)
            .map_err(|e| e.to_string())?;

        fs::write(&target_path, json)
            .map_err(|e| e.to_string())?;
        monitor_targets
    } else {
        let json = fs::read_to_string(&target_path)
        .map_err(|e| e.to_string())?;

        let monitor_targets: Vec<MonitorTarget> = serde_json::from_str(&json)
            .map_err(|e| e.to_string())?;
        monitor_targets
    };
    let state = app.state::<Mutex<MonitorState>>();
    let mut monitor_state = state.lock().unwrap();
    monitor_state.targets = targets;
    // save_monitors(app, &monitor_state.targets)?;
    Ok(())
}

pub fn save_monitors(
    app: &tauri::AppHandle,
    targets: &[MonitorTarget],
) -> Result<(), String> {
    let path = get_config_path(app)?
        .join("monitors.json");

    let json = serde_json::to_string_pretty(targets)
        .map_err(|e| e.to_string())?;

    fs::write(path, json)
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn initialize_monitor_thread(app: tauri::AppHandle) {
    let setting = load_settings(&app).unwrap();
    let timeout_secs = setting.monitoring.slow_response_threshold_ms / 1000;
    let interval_secs = setting.monitoring.engine_tick_seconds;
    std::thread::spawn(move || {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(timeout_secs))
            .build()
            .unwrap();

        loop {
            run_monitor_cycle(&app, &client);
            std::thread::sleep(std::time::Duration::from_secs(interval_secs));
        }
    });
}

pub fn run_monitor_cycle(app: &tauri::AppHandle, client: &reqwest::blocking::Client) {
    let targets = {
        let state = app.state::<Mutex<MonitorState>>();
        let state = state.lock().unwrap();
        state.targets.clone()
    };
    let now = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs();

    for target in targets {
        let mut last_error_status = None;
        let check_flag = {
            let state = app.state::<Mutex<MonitorState>>();
            let state = state.lock().unwrap();
            match state.statuses.get(&target.id){
                None => true,
                Some(status) => {
                    last_error_status = status.last_error.clone();
                    match status.last_checked {
                        None => true,
                        Some(last_checked) => now - last_checked > target.interval_seconds,
                    }
                }
            }
        };
        if check_flag {
            let start = Instant::now();
            let result = client.get(&target.endpoint).header("Connection", "close").send();
            let elapsed = start.elapsed().as_millis() as u64;
            let status_result;
            if result.is_ok() {
                status_result = MonitorStatus {
                    id: target.id.clone(),
                    healthy: result.unwrap().status().is_success(),
                    response_time_ms: Some(elapsed),
                    last_checked: Some(now),
                    last_error: last_error_status,
                };
            } else {
                println!("[{:?}] Monitor check failed for target {}: {:?}", now, target.id, &result.as_ref().err().unwrap());
                status_result = MonitorStatus {
                    id: target.id.clone(),
                    healthy: false,
                    response_time_ms: Some(elapsed),
                    last_checked: Some(now),
                    last_error: result.err().map(|e| {
                        if e.is_timeout() {
                            "timeout".to_string()
                        } else if e.is_connect() {
                            "connection error".to_string()
                        } else {
                            e.to_string()
                        }
                    }),
                };
                
            }
            let state = app.state::<Mutex<MonitorState>>();
            let mut state = state.lock().unwrap();
            state.update_status(status_result);
        }
    }
    let update: Vec<MonitorStatus> = {
        let state = app.state::<Mutex<MonitorState>>();
        let state = state.lock().unwrap();
        state.statuses.values().cloned().collect()
    };
    app.emit("monitor-status-update", update).unwrap();
}