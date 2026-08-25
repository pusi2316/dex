pub mod claude_api;
pub mod claude_code_search;
pub mod router;

use claude_api::ask_claude_api;
use claude_code_search::search_obsidian_vault;
use router::should_search_vault;


pub async fn think_and_search(transcript: &str) -> Result<String, String> {
    let should_search = should_search_vault(&transcript).await.unwrap_or(false);

    let context = if should_search {
        search_obsidian_vault(&transcript).await.unwrap_or_default()
    } else {
        String::new()
    };

    let prompt = if context.is_empty() {
        transcript
    } else {
        format!("Context from my notes: {}\n\nQuestion: {}", context, transcript)
    };

    ask_claude_api(prompt, "claude-sonnet-4-6").await
 }