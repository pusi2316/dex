use crate::audio_modules::paths::recording_path;
use crate::audio_modules::whisper_state::WhisperState;
use hound::WavReader;
use tauri::State;
use whisper_rs::{FullParams, SamplingStrategy};

#[tauri::command]
pub fn transcribe(whisper: State<WhisperState>) -> Result<String, String> {
    let audio_path = recording_path();

    let mut reader = WavReader::open(&audio_path).map_err(|e| e.to_string())?;
    let samples: Vec<f32> = reader
        .samples::<i16>()
        .map(|s| s.map(|v| v as f32 / i16::MAX as f32))
        .collect::<Result<_, _>>()
        .map_err(|e| e.to_string())?;

    let mut state = whisper.context.create_state().map_err(|e| e.to_string())?;
    let params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    state.full(params, &samples).map_err(|e| e.to_string())?;

    let num_segments = state.full_n_segments().map_err(|e| e.to_string())?;
    let mut text = String::new();
    for i in 0..num_segments {
        text.push_str(&state.full_get_segment_text(i).map_err(|e| e.to_string())?);
    }

    let _ = std::fs::remove_file(&audio_path);
    Ok(text.trim().to_string())
}
