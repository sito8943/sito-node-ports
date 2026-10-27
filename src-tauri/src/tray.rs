use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::tray::TrayIconBuilder;
use tauri::Manager;
use tauri_plugin_opener::OpenerExt;

use crate::processes::{self, NodeProcess};

pub const TRAY_ID: &str = "main-tray";
const TRAY_TICK_SECS: u64 = 3;
const MENU_OPEN_PREFIX: &str = "open:";
const MENU_KILL_PREFIX: &str = "kill:";
const MENU_SHOW: &str = "show";
const MENU_QUIT: &str = "quit";

const TEXT_EMPTY: &str = "Ningún proceso Node escuchando";
const TEXT_KILL: &str = "Terminar proceso";
const TEXT_SHOW: &str = "Mostrar ventana";
const TEXT_QUIT: &str = "Salir";

fn header_text(count: usize) -> String {
    match count {
        1 => "1 proceso Node".to_string(),
        n => format!("{n} procesos Node"),
    }
}

/// "mi-app · :3000, :5173"
fn process_title(process: &NodeProcess) -> String {
    let ports = process
        .ports
        .iter()
        .map(|port| format!(":{port}"))
        .collect::<Vec<_>>()
        .join(", ");
    format!("{} · {ports}", process.label)
}

fn disabled_item(app: &tauri::AppHandle, text: &str) -> tauri::Result<MenuItem<tauri::Wry>> {
    MenuItem::new(app, text, false, None::<&str>)
}

fn process_submenu(
    app: &tauri::AppHandle,
    process: &NodeProcess,
) -> tauri::Result<Submenu<tauri::Wry>> {
    let submenu = Submenu::new(app, process_title(process), true)?;
    for port in &process.ports {
        submenu.append(&MenuItem::with_id(
            app,
            format!("{MENU_OPEN_PREFIX}{port}"),
            format!("Abrir http://localhost:{port}"),
            true,
            None::<&str>,
        )?)?;
    }
    submenu.append(&PredefinedMenuItem::separator(app)?)?;
    let detail = if process.script.is_empty() {
        format!("PID {}", process.pid)
    } else {
        format!("{} — PID {}", process.script, process.pid)
    };
    submenu.append(&disabled_item(app, &detail)?)?;
    if let Some(cwd) = &process.cwd {
        submenu.append(&disabled_item(app, cwd)?)?;
    }
    submenu.append(&PredefinedMenuItem::separator(app)?)?;
    submenu.append(&MenuItem::with_id(
        app,
        format!("{MENU_KILL_PREFIX}{}", process.pid),
        TEXT_KILL,
        true,
        None::<&str>,
    )?)?;
    Ok(submenu)
}

fn build_menu(
    app: &tauri::AppHandle,
    list: &[NodeProcess],
) -> tauri::Result<Menu<tauri::Wry>> {
    let menu = Menu::new(app)?;
    if list.is_empty() {
        menu.append(&disabled_item(app, TEXT_EMPTY)?)?;
    } else {
        menu.append(&disabled_item(app, &header_text(list.len()))?)?;
        for process in list {
            menu.append(&process_submenu(app, process)?)?;
        }
    }
    menu.append(&PredefinedMenuItem::separator(app)?)?;
    menu.append(&MenuItem::with_id(app, MENU_SHOW, TEXT_SHOW, true, None::<&str>)?)?;
    menu.append(&MenuItem::with_id(app, MENU_QUIT, TEXT_QUIT, true, None::<&str>)?)?;
    Ok(menu)
}

pub fn show_main_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

fn on_menu_event(app: &tauri::AppHandle, id: &str) {
    match id {
        MENU_QUIT => app.exit(0),
        MENU_SHOW => show_main_window(app),
        other => {
            if let Some(port) = other.strip_prefix(MENU_OPEN_PREFIX) {
                let _ = app
                    .opener()
                    .open_url(format!("http://localhost:{port}"), None::<&str>);
            } else if let Some(pid) = other.strip_prefix(MENU_KILL_PREFIX) {
                if let Ok(pid) = pid.parse() {
                    let _ = processes::kill(pid);
                    refresh_now(app);
                }
            }
        }
    }
}

fn apply(app: &tauri::AppHandle, list: Vec<NodeProcess>) {
    let handle = app.clone();
    let _ = app.run_on_main_thread(move || {
        let Some(tray) = handle.tray_by_id(TRAY_ID) else {
            return;
        };
        // En macOS el título va junto al icono: el número de procesos de un vistazo.
        let title = (!list.is_empty()).then(|| list.len().to_string());
        let _ = tray.set_title(title.as_deref());
        if let Ok(menu) = build_menu(&handle, &list) {
            let _ = tray.set_menu(Some(menu));
        }
    });
}

/// Refresco inmediato tras una acción (p. ej. terminar un proceso).
fn refresh_now(app: &tauri::AppHandle) {
    let handle = app.clone();
    std::thread::spawn(move || {
        // Da tiempo al proceso a soltar el puerto antes de volver a listar.
        std::thread::sleep(std::time::Duration::from_millis(500));
        apply(&handle, processes::list());
    });
}

/// Un único hilo para toda la vida de la app; no hace nada mientras no haya
/// bandeja, y solo reconstruye el menú cuando la lista cambia.
pub fn spawn_updater(app: &tauri::AppHandle) {
    let handle = app.clone();
    std::thread::spawn(move || {
        let mut previous: Option<Vec<NodeProcess>> = None;
        loop {
            if handle.tray_by_id(TRAY_ID).is_some() {
                let list = processes::list();
                if previous.as_ref() != Some(&list) {
                    previous = Some(list.clone());
                    apply(&handle, list);
                }
            } else {
                previous = None;
            }
            std::thread::sleep(std::time::Duration::from_secs(TRAY_TICK_SECS));
        }
    });
}

#[cfg(target_os = "macos")]
fn set_dock_visible(app: &tauri::AppHandle, visible: bool) {
    let policy = if visible {
        tauri::ActivationPolicy::Regular
    } else {
        tauri::ActivationPolicy::Accessory
    };
    let _ = app.set_activation_policy(policy);
}

#[cfg(not(target_os = "macos"))]
fn set_dock_visible(_app: &tauri::AppHandle, _visible: bool) {}

pub fn create(app: &tauri::AppHandle) -> tauri::Result<()> {
    if app.tray_by_id(TRAY_ID).is_some() {
        return Ok(());
    }
    let list = processes::list();
    let menu = build_menu(app, &list)?;
    let mut builder = TrayIconBuilder::with_id(TRAY_ID)
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| on_menu_event(app, event.id.as_ref()));
    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }
    if !list.is_empty() {
        builder = builder.title(list.len().to_string());
    }
    builder.build(app)?;
    // Con bandeja la app es un accesorio de la barra de menús: sin icono en el Dock.
    set_dock_visible(app, false);
    Ok(())
}

pub fn remove(app: &tauri::AppHandle) {
    let _ = app.remove_tray_by_id(TRAY_ID);
    set_dock_visible(app, true);
}
