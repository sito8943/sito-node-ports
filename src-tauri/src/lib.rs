mod i18n;
mod processes;
mod settings;
mod tray;

use processes::NodeProcess;
use settings::Settings;
use tauri::{Manager, RunEvent, WindowEvent};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use tauri_plugin_opener::OpenerExt;

/// ⌃⌥N: abre la ventana aunque el icono de la bandeja no se vea.
/// Debe coincidir con SHORTCUT_LABEL en src/shared/constants.ts.
const TOGGLE_SHORTCUT: (Modifiers, Code) = (Modifiers::CONTROL.union(Modifiers::ALT), Code::KeyN);

fn toggle_shortcut() -> Shortcut {
    Shortcut::new(Some(TOGGLE_SHORTCUT.0), TOGGLE_SHORTCUT.1)
}

#[tauri::command]
fn get_tray_enabled(app: tauri::AppHandle) -> bool {
    settings::load(&app).tray_enabled
}

/// Consentimiento del usuario: crea o quita el icono y lo recuerda.
#[tauri::command]
fn set_tray_enabled(app: tauri::AppHandle, enabled: bool) -> Result<(), String> {
    if enabled {
        tray::create(&app).map_err(|e| e.to_string())?;
    } else {
        tray::remove(&app);
    }
    settings::save(
        &app,
        &Settings {
            tray_enabled: enabled,
        },
    )
}

#[tauri::command]
async fn list_node_processes() -> Vec<NodeProcess> {
    processes::list()
}

#[tauri::command]
async fn kill_node_process(pid: u32) -> Result<(), String> {
    processes::kill(pid)
}

#[tauri::command]
fn open_port(app: tauri::AppHandle, port: u16) -> Result<(), String> {
    app.opener()
        .open_url(format!("http://localhost:{port}"), None::<&str>)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn hide_window(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    if *shortcut == toggle_shortcut() && event.state() == ShortcutState::Pressed {
                        tray::toggle_main_window(app);
                    }
                })
                .build(),
        )
        .setup(|app| {
            let handle = app.handle();
            // Si otra app ya tiene el atajo, se sigue sin él: no es motivo para no arrancar.
            if let Err(e) = handle.global_shortcut().register(toggle_shortcut()) {
                eprintln!("no se pudo registrar el atajo global: {e}");
            }
            // La ventana nace oculta: si ya hay permiso, la app arranca solo en
            // la barra de menús; si no, se muestra para pedirlo.
            if settings::load(handle).tray_enabled {
                tray::create(handle)?;
            } else {
                tray::show_main_window(handle);
            }
            tray::spawn_updater(handle);
            Ok(())
        })
        .on_window_event(|window, event| {
            // Con bandeja, cerrar la ventana solo la esconde; sin ella, sale.
            if let WindowEvent::CloseRequested { api, .. } = event {
                if window.app_handle().tray_by_id(tray::TRAY_ID).is_some() {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_tray_enabled,
            set_tray_enabled,
            list_node_processes,
            kill_node_process,
            open_port,
            hide_window
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app, event| {
            // Abrir la app ya abierta (Spotlight, Launchpad, Finder) muestra la
            // ventana: otra salida cuando la bandeja queda oculta.
            #[cfg(target_os = "macos")]
            if let RunEvent::Reopen { .. } = event {
                tray::show_main_window(app);
            }
            #[cfg(not(target_os = "macos"))]
            let _ = (app, event);
        });
}
