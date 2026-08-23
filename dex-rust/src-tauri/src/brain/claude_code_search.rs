use std::process::Command;
use std:env;

pub fn search_obsidian_vault(transcript: &str) -> Result<String, String> {
    let obsidian_vault_path = env::var("OBSIDIAN_VAULT_PATH").map_err(|_| "OBSIDIAN_VAULT_PATH environment variable not set".to_string())?;

    let prompt = format!(
        "You are a code search assistant. Search the Obsidian vault at '{}' for relevant information based on the following transcript:\n\n{}",
        obsidian_vault_path, transcript
    );

    let output = Command::new("claude")
        .arg("-p", &prompt)
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(format!("claude -p failed with status: {}", output.status));
    }

    let result = String::from_utf8(output.stdout).map_err(|e| format!("Failed to parse output: {}", e))?;
    Ok(result)  
}