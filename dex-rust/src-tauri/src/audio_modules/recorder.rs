use std::process::{Child, Command};
use std::sync::Mutex;
use tauri::{AppHandle, State};

pub struct RecordingState {
    pub recording_process: Mutex<Option<Child>>,
}

impl RecordingState {
    pub fn new() -> Self {
        RecordingState {
            recording_process: Mutex::new(None),
        }
    }
}

pub fn toggle_recording_internal(state: &RecordingState) -> Result<String, String> {
    let mut process_lock = state.recording_process.lock().map_err(|e| e.to_string())?;

    match process_lock.take() {
        Some(mut child) => {
            child.kill().map_err(|e| e.to_string())?;
            Ok("Recording stopped.".into())
        }
        None => {
            let child = Command::new("sox")
                .args(["-d", "-r", "16000", "recordings/dex_recording.wav"])
                .spawn()
                .map_err(|e| e.to_string())?;

            *process_lock = Some(child);
            Ok("Recording started.".into())
        }
    }
}

#[tauri::command]
pub fn toggle_recording(app_handle: AppHandle, state: State<RecordingState>) -> Result<String, String> {
    toggle_recording_internal(&state)
}
