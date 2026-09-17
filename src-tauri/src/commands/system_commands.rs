use crate::models::process_info::ProcessInfo;
use crate::services::process_service;

#[tauri::command]
pub fn get_top_processes() -> Vec<ProcessInfo> {
    process_service::get_top_processes(5)
}
