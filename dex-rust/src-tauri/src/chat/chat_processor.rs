pub fn chat_processor(text: &str) -> Result<String, String> {
    Ok(format!("You said: {}", text))
}
