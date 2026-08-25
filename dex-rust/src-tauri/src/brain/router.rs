use crate::brain::claude_api::ask_claude_api;

pub async fn should_search_vault(transcript: &str) -> Result<bool, String> {
    let prompt = format!(
        "Does answering this require searching personal notes/vault? \
         Reply with only 'yes' or 'no'.\n\nQuery: {}",
        transcript
    );

    let response = ask_claude_api(prompt, "claude-haiku-4-5-20251001").await?;
    Ok(response.trim().to_lowercase().starts_with("yes"))
}
