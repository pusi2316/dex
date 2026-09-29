use crate::brain;

use brain::claude_code_search::ask_claude;

#[tauri::command]
pub async fn process_chat(text: &str) -> Result<String, String> {
    let result = ask_claude(&text).await?;
    println!("claude result: {}", result);
    Ok(format!("the answer is: {}", result))
}
