use std::env;
use std::process::Command;

pub async fn ask_claude(prompt: &str) -> Result<String, String> {
    println!("ask_claude called with prompt: {}", prompt);
    let output = Command::new("claude")
        .args(["-p", &prompt])
        .output()
        .map_err(|e| e.to_string())?;
    println!("claude output: {:?}", output);
    if !output.status.success() {
        return Err(format!("claude -p failed with status: {}", output.status));
    }

    let result = String::from_utf8(output.stdout).map_err(|e| format!("Failed to parse output: {}", e))?;
    Ok(result)
}
