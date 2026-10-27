import { useContext } from "react";

import { PortsContext } from "./context";
import { PortsContextValue } from "./types";

export function usePorts(): PortsContextValue {
  const context = useContext(PortsContext);
  if (!context) {
    throw new Error("usePorts must be used within PortsProvider");
  }
  return context;
}
