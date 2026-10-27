import { invoke } from "@tauri-apps/api/core";

import { NodeProcess } from "@/shared/models";
import { PORTS_COMMAND } from "./constants";

/**
 * Único punto de la app que llama a `invoke` (ARCHITECTURE_RULES §13).
 * Los features consumen estos wrappers a través de sus managers.
 */

/** Si el usuario ya aceptó que la app viva en la barra de menús. */
export function getTrayEnabled(): Promise<boolean> {
  return invoke<boolean>(PORTS_COMMAND.GET_TRAY_ENABLED);
}

/** Crea o quita el icono de la barra de menús y guarda la decisión. */
export function setTrayEnabled(enabled: boolean): Promise<void> {
  return invoke<void>(PORTS_COMMAND.SET_TRAY_ENABLED, { enabled });
}

/** Procesos Node con al menos un puerto TCP en LISTEN. */
export function listNodeProcesses(): Promise<NodeProcess[]> {
  return invoke<NodeProcess[]>(PORTS_COMMAND.LIST_NODE_PROCESSES);
}

/** SIGTERM al proceso; el backend rechaza pids que no sean Node escuchando. */
export function killNodeProcess(pid: number): Promise<void> {
  return invoke<void>(PORTS_COMMAND.KILL_NODE_PROCESS, { pid });
}

/** Abre http://localhost:<port> en el navegador por defecto. */
export function openPort(port: number): Promise<void> {
  return invoke<void>(PORTS_COMMAND.OPEN_PORT, { port });
}

export function hideWindow(): Promise<void> {
  return invoke<void>(PORTS_COMMAND.HIDE_WINDOW);
}
