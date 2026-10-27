import { api } from "@/shared/services";
import { NodeProcess } from "@/shared/models";

/**
 * Operaciones de dominio sobre la bandeja y los procesos Node: los componentes
 * hablan con este manager, nunca con la capa de IPC (ARCHITECTURE_RULES §3).
 */
export class PortsManager {
  isTrayEnabled(): Promise<boolean> {
    return api.getTrayEnabled();
  }

  setTrayEnabled(enabled: boolean): Promise<void> {
    return api.setTrayEnabled(enabled);
  }

  list(): Promise<NodeProcess[]> {
    return api.listNodeProcesses();
  }

  kill(pid: number): Promise<void> {
    return api.killNodeProcess(pid);
  }

  open(port: number): Promise<void> {
    return api.openPort(port);
  }

  hideWindow(): Promise<void> {
    return api.hideWindow();
  }
}
