import React from "react";
import ReactDOM from "react-dom/client";

import { Ports, PortsProvider } from "@/features/ports";
import { ThemeProvider } from "@/shared/providers/ThemeProvider";
import { AppLayout } from "@/app/layout/AppLayout";
import { language } from "@/lang";

import "@sito/ui/theme.css";
import "@sito/ui/styles.css";
import "@/styles/index.css";

document.documentElement.lang = language;

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ThemeProvider>
      <PortsProvider>
        <AppLayout>
          <Ports />
        </AppLayout>
      </PortsProvider>
    </ThemeProvider>
  </React.StrictMode>,
);
