/**
 * Contrato de todos los diccionarios: cada idioma define las mismas claves,
 * y las cadenas con datos son funciones (ARCHITECTURE_RULES §9).
 */
export interface Dictionary {
  appName: string;
  consentTitle: string;
  consentText: string;
  consentHint: string;
  shortcutHint: (shortcut: string) => string;
  consentAccept: string;
  trayActive: string;
  trayDisable: string;
  hideWindow: string;
  processesTitle: string;
  emptyTitle: string;
  emptyHint: string;
  openPort: (port: number) => string;
  pidLabel: (pid: number) => string;
  kill: string;
  dismiss: string;
  errorProcessNotFound: string;
  errorKillFailed: string;
  errorGeneric: string;
}
