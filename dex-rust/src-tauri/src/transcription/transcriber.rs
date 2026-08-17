use std::env;
use std::fs;
use std::process::Command;

#[tauri::command]
pub fn transcribe(audio_path: String) -> Result<String, String> {
    let whisper_binary =
        env::var("WHISPER_BINARY").map_err(|_| "WHISPER_BINARY not set in .env".to_string())?;
    let model_path =
        env::var("WHISPER_MODEL").map_err(|_| "WHISPER_MODEL not set in .env".to_string())?;

    let status = Command::new(&whisper_binary)
        .args(["-m", &model_path, "-f", &audio_path, "-ng", "-otxt"])
        .status()
        .map_err(|e| e.to_string())?;

    if !status.success() {
        return Err(format!("whisper-cli exited with status: {}", status));
    }

    let output_txt_path = format!("{}.txt", audio_path);
    let text = fs::read_to_string(&output_txt_path).map_err(|e| e.to_string())?;

    Ok(text.trim().to_string())
}
