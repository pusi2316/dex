use crate::audio_modules::whisper_state::WhisperState;
use hound::WavReader;
use tauri::State;
use whisper_rs::{FullParams, SamplingStrategy, WhisperError};

#[tauri::command]
pub fn transcribe(whisper: State<WhisperState>) -> Result<String, String> {
    let audio_path = "recordings/dex_recording.wav";

    let mut reader = WavReader::open(&audio_path).map_err(|e: hound::Error| e.to_string())?;

    let num_samples = reader.duration() as usize;

    let samples = reader
        .samples::<i16>()
        .take(num_samples)
        .map(|s| s.map(|v| v as f32 / i16::MAX as f32))
        .collect::<Result<Vec<f32>, hound::Error>>() // <-- Specify Vec<f32> here
        .map_err(|e| e.to_string() + " transcribe Error")?;

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
