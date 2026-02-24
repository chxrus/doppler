// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod audio;
mod commands;
pub mod gemini;
pub mod models;
mod storage;

use std::sync::Mutex;
use tauri::{Emitter, Manager};
use tauri_plugin_global_shortcut::ShortcutState;

#[derive(Default)]
struct WindowInteractionState {
    click_through: Mutex<bool>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn set_capture_visibility(app: tauri::AppHandle, hide_from_capture: bool) -> Result<(), String> {
    let main_window = app
        .get_webview_window("main")
        .ok_or_else(|| "Main window not found".to_string())?;

    main_window
        .set_content_protected(hide_from_capture)
        .map_err(|error| error.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(WindowInteractionState::default())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            #[cfg(desktop)]
            {
                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_shortcuts(["CommandOrControl+Shift+X"])?
                        .with_handler(|app, _shortcut, event| {
                            if event.state == ShortcutState::Pressed {
                                let state = app.state::<WindowInteractionState>();
                                let next_click_through_state =
                                    if let Ok(mut current_state) = state.click_through.lock() {
                                        *current_state = !*current_state;
                                        *current_state
                                    } else {
                                        false
                                    };

                                if let Some(main_window) = app.get_webview_window("main") {
                                    let _ = main_window
                                        .set_ignore_cursor_events(next_click_through_state);
                                    let _ =
                                        app.emit("click-through-changed", next_click_through_state);
                                }
                            }
                        })
                        .build(),
                )?;
            }

            if let Some(main_window) = app.get_webview_window("main") {
                // Best-effort: hide window contents from OS-level capture tools where supported.
                let _ = main_window.set_content_protected(true);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            set_capture_visibility,
            commands::save_api_key,
            commands::get_api_key,
            commands::send_message,
            commands::start_recording,
            commands::stop_recording_and_transcribe,
            commands::stop_recording,
            commands::transcribe_last_recording,
            commands::list_recording_devices,
            commands::speak_text,
            commands::stop_speaking,
            commands::is_speaking,
            commands::get_settings,
            commands::update_settings,
            commands::set_window_always_on_top,
            commands::set_window_click_through
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
