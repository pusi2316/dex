// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    dex_rust_lib::run().invoke_handler(tauri::generate_handler![
        dex_rust_lib::audio_modules::speaker::speak,
    ])
}
