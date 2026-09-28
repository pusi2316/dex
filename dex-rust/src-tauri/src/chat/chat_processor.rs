#[tauri::command]
pub fn process_chat(text: &str) -> Result<String, String> {
    Ok(format!("You said: {}", text))
}
