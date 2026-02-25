// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod audio;
mod commands;
pub mod gemini;
pub mod models;
pub mod ollama;
mod storage;

use std::sync::Mutex;
use tauri::{Emitter, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

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
                let toggle_click_through_shortcut: tauri_plugin_global_shortcut::Shortcut =
                    "CmdOrCtrl+Shift+X".parse()?;
                let toggle_visibility_shortcut: tauri_plugin_global_shortcut::Shortcut =
                    "CmdOrCtrl+Shift+Space".parse()?;
                let toggle_recording_shortcut: tauri_plugin_global_shortcut::Shortcut =
                    "CmdOrCtrl+R".parse()?;

                let toggle_click_through_id = toggle_click_through_shortcut.id();
                let toggle_visibility_id = toggle_visibility_shortcut.id();
                let toggle_recording_id = toggle_recording_shortcut.id();

                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |app, shortcut, event| {
                            if event.state == ShortcutState::Pressed {
                                let shortcut_id = shortcut.id();
                                if shortcut_id == toggle_click_through_id {
                                    // Toggle click-through
                                    let state = app.state::<WindowInteractionState>();
                                    let next_click_through_state =
                                        if let Ok(mut current_state) = state.click_through.lock() {
                                            *current_state = !*current_state;
                                            *current_state
                                        } else {
                                            false
                                        };

                                    if let Some(main_window) = app.get_webview_window("main") {
                                        let _ =
                                            main_window.set_ignore_cursor_events(next_click_through_state);
                                        let _ =
                                            app.emit("click-through-changed", next_click_through_state);
                                    }
                                } else if shortcut_id == toggle_visibility_id {
                                    // Toggle window visibility
                                    if let Some(main_window) = app.get_webview_window("main") {
                                        if let Ok(is_visible) = main_window.is_visible() {
                                            if is_visible {
                                                let _ = main_window.hide();
                                            } else {
                                                let _ = main_window.show();
                                                let _ = main_window.set_focus();
                                            }
                                        }
                                    }
                                } else if shortcut_id == toggle_recording_id {
                                    // Toggle recording
                                    let _ = app.emit("hotkey-toggle-recording", ());
                                }
                            }
                        })
                        .build(),
                )?;

                let shortcuts_to_register = [
                    ("toggle click-through", toggle_click_through_shortcut),
                    ("toggle window visibility", toggle_visibility_shortcut),
                    ("toggle recording", toggle_recording_shortcut),
                ];

                let mut failed_shortcuts: Vec<String> = Vec::new();
                let mut registered_count = 0usize;

                for (label, shortcut) in shortcuts_to_register {
                    if let Err(error) = app.global_shortcut().register(shortcut) {
                        failed_shortcuts.push(format!("{label} ({shortcut}): {error}"));
                    } else {
                        registered_count += 1;
                    }
                }

                if !failed_shortcuts.is_empty() {
                    let _ = app.emit("global-shortcut-registration-failed", failed_shortcuts.clone());
                    eprintln!(
                        "Failed to register some global shortcuts:\n- {}",
                        failed_shortcuts.join("\n- ")
                    );
                }

                if registered_count == 0 {
                    return Err("No global shortcuts could be registered. Check for shortcut conflicts in other apps or OS settings.".into());
                }
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
            commands::list_ollama_models,
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
            commands::set_window_click_through,
            commands::set_screen_capture_protection
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
