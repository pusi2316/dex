use crate::brain;

use brain::claude_code_search::ask_claude;

#[tauri::command]
pub async fn process_chat(text: &str) -> Result<String, String> {
    let result = ask_claude(&text).await;

    let body: serde_json::Value = result?.json().await.map_err(|e| e.to_string())?;
    let text: String = body["content"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|b| b["type"] == "text")
        .filter_map(|b| b["text"].as_str())
        .collect::<Vec<_>>()
        .join("\n");

    if text.trim().is_empty() {
        return Err(format!("unexpected response shape: {body}"));
    }

    println!("claude result: {}", text);
    Ok(format!("{}", text))
}
