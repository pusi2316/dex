use std::process::Command;

#[tauri::command]
pub fn speak(text: String, voice: Option<String>) -> Result<(), String> {
    let status = Command::new("say")
        .arg("-v")
        .arg(voice.unwrap_or_else(|| "Alex".to_string()))
        .arg(&text)
        .status()
        .map_err(|e| e.to_string())?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("Failed to speak: {:?}", status))
    }
}
