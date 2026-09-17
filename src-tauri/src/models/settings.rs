use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppSettings {
    pub general: GeneralSettings,
    pub appearance: AppearanceSettings,
    pub monitoring: MonitoringSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    pub active_profile: String,
    pub click_through: bool,
    pub pinned: bool,
    pub run_on_startup: bool,
    pub close_on_exit: bool,

    pub window_x: Option<i32>,
    pub window_y: Option<i32>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceSettings {
    pub opacity: f32,
    pub theme: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringSettings {
    pub engine_tick_seconds: u64,
    pub slow_response_threshold_ms: u64,
    pub history_point: usize,
    pub failure_threshold: u32,
    pub logging_enabled: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            general: GeneralSettings {
                active_profile: "default".to_string(),
                click_through: false,
                pinned: false,
                run_on_startup: false,
                close_on_exit: false,
                window_x: None,
                window_y: None,
            },
            appearance: AppearanceSettings {
                opacity: 0.5,
                theme: "dark".to_string(),
            },
            monitoring: MonitoringSettings {
                engine_tick_seconds: 1,
                slow_response_threshold_ms: 10000,
                history_point: 16,
                failure_threshold: 3,
                logging_enabled: true,
            },
        }
    }
}
