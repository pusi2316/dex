use std::process::Command;

#[tauri::command]
fn speak(text: String) -> Result<(), String> {
    let status = Command::new("say")
        .arg(&text)
        .status()
        .map_err(|e| e.to_string())?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("Failed to speak: {:?}", status))
    }
}
