import { createContext } from "react";

import { PortsContextValue } from "./types";

export const PortsContext = createContext<PortsContextValue | null>(null);
