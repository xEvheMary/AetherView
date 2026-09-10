export interface MonitorState {
  targets: TargetMonitor[];
  statuses: { [key: string]: MonitorStatus };
}

export interface TargetMonitor {
  id: string;
  name: string;
  method: string;
  endpoint: string;
  interval_seconds: number;
  enabled: boolean;
}

export interface MonitorStatus {
  id: string;
  healthy: boolean;
  response_time_ms: number | null;
  last_checked: number | null;
  last_error: string | null;
}
