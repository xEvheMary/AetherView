export interface Settings {
  general: GeneralSettings;
  appearance: AppearanceSettings;
  monitoring: MonitoringSettings;
}
export interface GeneralSettings {
  type: "general";
  active_profile: string;
  click_through: boolean;
  pinned: boolean;
  run_on_startup: boolean;
  close_on_exit: boolean;
}
export interface AppearanceSettings {
  type: "appearance";
  opacity: number;
  theme: string;
}
export interface MonitoringSettings {
  type: "monitoring";
  engine_tick_seconds: number;
  slow_response_threshold_ms: number;
  logging_enabled: boolean;
}
