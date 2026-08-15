use std::process::Command;

#[tauri::command]
fn speak() -> Result<(), String> {}
