/** Espejo de `NodeProcess` en src-tauri/src/processes.rs. */
export interface NodeProcess {
  pid: number;
  /** Nombre del package.json del proyecto, o la carpeta donde corre. */
  label: string;
  /** Herramienta que lo lanzó: "vite", "next dev"… Vacío si no se dedujo. */
  script: string;
  cwd: string | null;
  ports: number[];
}
