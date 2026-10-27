import { NodeProcess } from "@/shared/models";

export interface PortsContextValue {
  /** `null` mientras se lee la preferencia guardada. */
  trayEnabled: boolean | null;
  processes: NodeProcess[];
  busyPid: number | null;
  error: string | null;
  enableTray: () => Promise<void>;
  disableTray: () => Promise<void>;
  kill: (pid: number) => Promise<void>;
  open: (port: number) => Promise<void>;
  hideWindow: () => Promise<void>;
  clearError: () => void;
}
