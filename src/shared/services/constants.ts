// Nombres de comandos Tauri (deben coincidir con src-tauri/src/lib.rs)
export const PORTS_COMMAND = {
  GET_TRAY_ENABLED: "get_tray_enabled",
  SET_TRAY_ENABLED: "set_tray_enabled",
  LIST_NODE_PROCESSES: "list_node_processes",
  KILL_NODE_PROCESS: "kill_node_process",
  OPEN_PORT: "open_port",
  HIDE_WINDOW: "hide_window",
} as const;

export type PortsCommand = (typeof PORTS_COMMAND)[keyof typeof PORTS_COMMAND];
