use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub active_profile: String,
    pub click_through: bool,
    pub pinned: bool,
    pub opacity: f32,

    pub window_x: Option<i32>,
    pub window_y: Option<i32>,
}

impl AppSettings {
    pub fn default() -> Self {
        AppSettings {
            active_profile: "default".to_string(),
            click_through: false,
            pinned: false,
            opacity: 0.9,
            window_x: None,
            window_y: None,
        }
    }
}