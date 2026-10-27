import {
  ReactNode,
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";

import { NodeProcess } from "@/shared/models";
import { PortsManager } from "@/features/ports/managers/PortsManager";
import { translateError } from "@/features/ports/utils";
import { PortsContext } from "./context";
import { PROCESS_POLL_INTERVAL_MS } from "./constants";
import { PortsContextValue } from "./types";

export function PortsProvider({ children }: { children: ReactNode }) {
  const managerRef = useRef(new PortsManager());
  const [trayEnabled, setTrayEnabled] = useState<boolean | null>(null);
  const [processes, setProcesses] = useState<NodeProcess[]>([]);
  const [busyPid, setBusyPid] = useState<number | null>(null);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    try {
      setProcesses(await managerRef.current.list());
    } catch (e) {
      setError(translateError(String(e)));
    }
  }, []);

  useEffect(() => {
    (async () => {
      try {
        setTrayEnabled(await managerRef.current.isTrayEnabled());
      } catch (e) {
        setError(translateError(String(e)));
        setTrayEnabled(false);
      }
    })();
  }, []);

  // La lista solo se ve con la bandeja activa: sin ella no se sondea.
  useEffect(() => {
    if (!trayEnabled) return;
    refresh();
    const id = setInterval(refresh, PROCESS_POLL_INTERVAL_MS);
    return () => clearInterval(id);
  }, [trayEnabled, refresh]);

  const changeTray = useCallback(async (enabled: boolean) => {
    setError(null);
    try {
      await managerRef.current.setTrayEnabled(enabled);
      setTrayEnabled(enabled);
      // Aceptar = la app pasa a vivir en la barra de menús.
      if (enabled) await managerRef.current.hideWindow();
    } catch (e) {
      setError(translateError(String(e)));
    }
  }, []);

  const enableTray = useCallback(() => changeTray(true), [changeTray]);
  const disableTray = useCallback(() => changeTray(false), [changeTray]);

  const kill = useCallback(
    async (pid: number) => {
      setBusyPid(pid);
      setError(null);
      try {
        await managerRef.current.kill(pid);
        await refresh();
      } catch (e) {
        setError(translateError(String(e)));
      } finally {
        setBusyPid(null);
      }
    },
    [refresh],
  );

  const open = useCallback(async (port: number) => {
    try {
      await managerRef.current.open(port);
    } catch (e) {
      setError(translateError(String(e)));
    }
  }, []);

  const hideWindow = useCallback(() => managerRef.current.hideWindow(), []);

  const clearError = useCallback(() => setError(null), []);

  const value = useMemo<PortsContextValue>(
    () => ({
      trayEnabled,
      processes,
      busyPid,
      error,
      enableTray,
      disableTray,
      kill,
      open,
      hideWindow,
      clearError,
    }),
    [
      trayEnabled,
      processes,
      busyPid,
      error,
      enableTray,
      disableTray,
      kill,
      open,
      hideWindow,
      clearError,
    ],
  );

  return (
    <PortsContext.Provider value={value}>{children}</PortsContext.Provider>
  );
}
