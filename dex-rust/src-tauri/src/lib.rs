mod audio_modules;
mod transcription;

use audio_modules::recorder::{toggle_recording, toggle_recording_internal, RecordingState};
use audio_modules::speaker::speak;
use tauri::Manager;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};
use transcription::transcriber::transcribe;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenvy::dotenv().ok();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .manage(RecordingState::new())
        .setup(|app| {
            let _handle = app.handle().clone();
            app.global_shortcut().on_shortcut(
                "CommandOrControl+Shift+Space",
                move |app_handle, _shortcut, event| {
                    if event.state() == ShortcutState::Pressed {
                        let state = app_handle.state::<RecordingState>();
                        match toggle_recording_internal(&state) {
                            Ok(status) => println!("Recording: {}", status),
                            Err(e) => eprintln!("Recording error: {}", e),
                        }
                    }
                },
            )?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            speak,
            toggle_recording,
            transcribe
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
