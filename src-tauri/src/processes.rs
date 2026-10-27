use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use std::process::Command;

/// Nombres de ejecutable que cuentan como "proceso Node".
const NODE_COMMANDS: [&str; 3] = ["node", "bun", "deno"];
/// Cuántas carpetas sube buscando un package.json desde el cwd del proceso.
const PACKAGE_JSON_MAX_DEPTH: usize = 4;
/// Entradas genéricas: el nombre útil es la carpeta del paquete, no el archivo.
const GENERIC_ENTRIES: [&str; 5] = ["index", "cli", "bin", "main", "server"];

#[derive(Serialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeProcess {
    pub pid: u32,
    /// Nombre del package.json del proyecto, o la carpeta donde corre.
    pub label: String,
    /// Herramienta que lanzó el proceso: "vite", "next dev", "server.js"…
    pub script: String,
    pub cwd: Option<String>,
    pub ports: Vec<u16>,
}

/// Puerto de un NAME de lsof: "*:3000", "127.0.0.1:5173", "[::1]:5173".
fn parse_port(name: &str) -> Option<u16> {
    name.rsplit(':').next()?.parse().ok()
}

/// `lsof -F pcn` → pid → (comando, puertos). lsof sale con 1 cuando no
/// encuentra nada, así que se lee stdout sin mirar el código de salida.
fn listening_sockets() -> BTreeMap<u32, (String, BTreeSet<u16>)> {
    let mut result = BTreeMap::new();
    let Ok(output) = Command::new("lsof")
        .args(["+c0", "-nP", "-iTCP", "-sTCP:LISTEN", "-Fpcn"])
        .output()
    else {
        return result;
    };
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut pid: Option<u32> = None;
    let mut command = String::new();
    for line in stdout.lines() {
        let (tag, value) = line.split_at(line.len().min(1));
        match tag {
            "p" => {
                pid = value.parse().ok();
                command.clear();
            }
            "c" => command = value.to_string(),
            "n" => {
                let (Some(pid), Some(port)) = (pid, parse_port(value)) else {
                    continue;
                };
                result
                    .entry(pid)
                    .or_insert_with(|| (command.clone(), BTreeSet::new()))
                    .1
                    .insert(port);
            }
            _ => {}
        }
    }
    result
}

fn is_node_command(command: &str) -> bool {
    NODE_COMMANDS.contains(&command.to_ascii_lowercase().as_str())
}

fn pid_list(pids: &[u32]) -> String {
    pids.iter()
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

/// Directorio de trabajo de cada pid, en una sola llamada a lsof.
fn working_dirs(pids: &[u32]) -> BTreeMap<u32, String> {
    let mut result = BTreeMap::new();
    let Ok(output) = Command::new("lsof")
        .args(["-a", "-d", "cwd", "-Fn", "-p", &pid_list(pids)])
        .output()
    else {
        return result;
    };
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut pid: Option<u32> = None;
    for line in stdout.lines() {
        if let Some(value) = line.strip_prefix('p') {
            pid = value.parse().ok();
        } else if let (Some(value), Some(pid)) = (line.strip_prefix('n'), pid) {
            result.insert(pid, value.to_string());
        }
    }
    result
}

/// Línea de comandos completa de cada pid.
fn command_lines(pids: &[u32]) -> BTreeMap<u32, String> {
    let Ok(output) = Command::new("ps")
        .args(["-o", "pid=,command=", "-p", &pid_list(pids)])
        .output()
    else {
        return BTreeMap::new();
    };
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| {
            let (pid, command) = line.trim_start().split_once(' ')?;
            Some((pid.parse().ok()?, command.trim().to_string()))
        })
        .collect()
}

fn file_stem(path: &str) -> String {
    let path = Path::new(path);
    let stem = path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();
    if !GENERIC_ENTRIES.contains(&stem.as_str()) {
        return stem;
    }
    // ".../next/dist/bin/next" ya sale bien; ".../vite/bin/index.js" no:
    // se busca la primera carpeta que no sea dist/bin/src.
    path.ancestors()
        .skip(1)
        .filter_map(|dir| dir.file_name())
        .map(|name| name.to_string_lossy().to_string())
        .find(|name| !matches!(name.as_str(), "dist" | "bin" | "src" | "lib"))
        .unwrap_or(stem)
}

/// "node --inspect /x/node_modules/.bin/next dev -p 3000" → "next dev".
/// Rutas con espacios se parten mal; para una etiqueta corta es aceptable.
fn script_from_command(command: &str) -> String {
    let mut args = command
        .split_whitespace()
        .skip(1)
        .skip_while(|arg| arg.starts_with('-'));
    let Some(entry) = args.next() else {
        return String::new();
    };
    let mut script = file_stem(entry);
    if let Some(sub) = args.next() {
        let looks_like_subcommand = !sub.starts_with('-')
            && !sub.contains('/')
            && !sub.contains('.')
            && sub.parse::<u16>().is_err();
        if looks_like_subcommand {
            script = format!("{script} {sub}");
        }
    }
    script
}

/// `name` del package.json más cercano subiendo desde `cwd`.
fn package_name(cwd: &str) -> Option<String> {
    Path::new(cwd)
        .ancestors()
        .take(PACKAGE_JSON_MAX_DEPTH)
        .find_map(|dir| {
            let content = fs::read_to_string(dir.join("package.json")).ok()?;
            let json: serde_json::Value = serde_json::from_str(&content).ok()?;
            json.get("name")?.as_str().map(str::to_string)
        })
}

fn label_for(cwd: Option<&str>, script: &str, command: &str) -> String {
    if let Some(cwd) = cwd {
        if let Some(name) = package_name(cwd) {
            return name;
        }
        if let Some(dir) = Path::new(cwd).file_name() {
            if cwd != "/" {
                return dir.to_string_lossy().to_string();
            }
        }
    }
    if script.is_empty() {
        command.to_string()
    } else {
        script.to_string()
    }
}

/// Procesos Node con al menos un puerto TCP en LISTEN, ordenados por puerto.
pub fn list() -> Vec<NodeProcess> {
    let sockets: BTreeMap<_, _> = listening_sockets()
        .into_iter()
        .filter(|(_, (command, _))| is_node_command(command))
        .collect();
    if sockets.is_empty() {
        return Vec::new();
    }
    let pids: Vec<u32> = sockets.keys().copied().collect();
    let cwds = working_dirs(&pids);
    let commands = command_lines(&pids);

    let mut processes: Vec<NodeProcess> = sockets
        .into_iter()
        .map(|(pid, (command, ports))| {
            let cwd = cwds.get(&pid).cloned();
            let script = commands
                .get(&pid)
                .map(|line| script_from_command(line))
                .unwrap_or_default();
            NodeProcess {
                pid,
                label: label_for(cwd.as_deref(), &script, &command),
                script,
                cwd,
                ports: ports.into_iter().collect(),
            }
        })
        .collect();
    processes.sort_by_key(|p| p.ports.first().copied().unwrap_or(u16::MAX));
    processes
}

/// SIGTERM, solo a pids que siguen siendo procesos Node escuchando: así la UI
/// no puede usarse para matar cualquier cosa por pid.
pub fn kill(pid: u32) -> Result<(), String> {
    if !list().iter().any(|p| p.pid == pid) {
        return Err("process-not-found".into());
    }
    let status = Command::new("kill")
        .args(["-TERM", &pid.to_string()])
        .status()
        .map_err(|e| e.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err("kill-failed".into())
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_port, script_from_command};

    #[test]
    fn parses_ports() {
        assert_eq!(parse_port("*:3000"), Some(3000));
        assert_eq!(parse_port("127.0.0.1:5173"), Some(5173));
        assert_eq!(parse_port("[::1]:5173"), Some(5173));
        assert_eq!(parse_port("nonsense"), None);
    }

    #[test]
    fn derives_script() {
        assert_eq!(
            script_from_command("node /p/node_modules/.bin/vite"),
            "vite"
        );
        assert_eq!(
            script_from_command("node --inspect /p/node_modules/next/dist/bin/next dev -p 3000"),
            "next dev"
        );
        assert_eq!(script_from_command("node server.js"), "server");
        assert_eq!(
            script_from_command("node /p/node_modules/vite/bin/index.js"),
            "vite"
        );
        assert_eq!(script_from_command("node"), "");
    }
}
