use crate::audio_modules::whisper_state::WhisperState;
use hound::WavReader;
use tauri::State;
use whisper_rs::{FullParams, SamplingStrategy, WhisperError};

#[tauri::command]
pub fn transcribe(whisper: State<WhisperState>) -> Result<String, String> {
    let audio_path = "recordings/dex_recording.wav";

    let mut reader = WavReader::open(&audio_path).map_err(|e: hound::Error| e.to_string())?;
    let samples: Vec<f32> = reader
        .samples::<i16>()
        .map(|s| s.map(|v| v as f32 / i16::MAX as f32))
        .collect::<Result<_, _>>()
        .map_err(|e| e.to_string())?;

    let mut state = whisper
        .context
        .create_state()
        .map_err(|e: WhisperError| e.to_string())?;
    let params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    state.full(params, &samples).map_err(|e: WhisperError| e.to_string())?;

    let num_segments = state.full_n_segments();
    let mut text = String::new();
    for i in 0..num_segments {
        text.push_str(
            &state
                .get_segment(i)
                .expect("Failed to get Segment")
                .to_str()
                .expect("Failed to convert to str"),
        );
    }

    let _ = std::fs::remove_file(&audio_path);
    Ok(text.trim().to_string())
}
