use std::env;
use whisper_rs::{WhisperContext, WhisperContextParameters};

pub struct WhisperState {
    pub context: WhisperContext,
}

impl WhisperState {
    pub fn new() -> Self {
        let model_path = env::var("WHISPER_MODEL").expect("WHISPER_MODEL environment variable not set");
        let context = WhisperContext::new_with_params(&model_path, WhisperContextParameters::default())
            .expect("Failed to create Whisper context");
        WhisperState { context }
    }
}
