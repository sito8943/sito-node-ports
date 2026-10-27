import type { Dictionary } from "./types";

export const es: Dictionary = {
  appName: "Sito Node Ports",
  consentTitle: "¿Te pongo en la barra de menús?",
  consentText:
    "Sito Node Ports vive junto al reloj. Al pulsar el icono verás cada proceso Node que tiene un puerto abierto, con el nombre de su proyecto.",
  consentHint:
    "La ventana se esconderá y la app dejará de aparecer en el Dock. Puedes quitarla cuando quieras desde aquí.",
  shortcutHint: (shortcut: string) =>
    `¿No ves el icono? Pulsa ${shortcut} desde cualquier app, o vuelve a abrir Sito Node Ports desde Spotlight o Launchpad.`,
  consentAccept: "Sí, ponla en la barra de menús",
  trayActive: "Activa en la barra de menús",
  trayDisable: "Quitar de la barra de menús",
  hideWindow: "Esconder ventana",
  processesTitle: "Procesos Node con puertos",
  emptyTitle: "Nada escuchando",
  emptyHint: "No hay procesos Node con puertos TCP abiertos ahora mismo.",
  openPort: (port: number) => `Abrir localhost:${port}`,
  pidLabel: (pid: number) => `PID ${pid}`,
  kill: "Terminar proceso",
  dismiss: "Cerrar",
  errorProcessNotFound: "Ese proceso ya no está escuchando.",
  errorKillFailed: "No se pudo terminar el proceso.",
  errorGeneric: "Algo salió mal:",
};
