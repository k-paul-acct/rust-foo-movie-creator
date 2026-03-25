use serde::{Deserialize, Serialize};
use std::{sync::{
    Arc, Mutex, atomic::{AtomicBool, Ordering}
}, thread, time::Duration};
use tauri::{AppHandle, Emitter, State};

use crate::ffmpeg::{ffmpeg_version, run_generation, ProgressReporter};

pub struct GenerationState {
    pub cancelled: Arc<AtomicBool>,
}

#[derive(Debug, Deserialize)]
pub struct ScreensaverConfig {
    pub duration: f32,
    pub shape_type: String,
    pub shape_count: u32,
    pub min_size: u32,
    pub max_size: u32,
    pub min_speed: u32,
    pub max_speed: u32,
    pub bg_r: u8,
    pub bg_g: u8,
    pub bg_b: u8,
    pub colors: Vec<[u8; 4]>,
    pub blur_edges: bool,
    pub seed: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct GenerateConfig {
    pub output_path: String,
    pub codec: String,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub quality: u32,
    pub transition: String,
    pub transition_dur: f32,
    pub images: Vec<String>,
    pub effects: Vec<String>,
    pub min_dur: f32,
    pub max_dur: f32,
    pub total_dur: Option<f32>,
    pub seed: Option<u32>,
    pub no_repeat: bool,
    pub screensaver: Option<ScreensaverConfig>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProgressPayload {
    pub phase: String,
    pub current_frame: u32,
    pub total_frames: u32,
    pub percentage: f32,
    pub message: String,
}

pub type SharedGenerationState = Mutex<GenerationState>;

impl Default for GenerationState {
    fn default() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }
}

#[tauri::command]
pub fn get_ffmpeg_version() -> Result<String, String> {
    ffmpeg_version().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_images_in_dir(dir: &str) -> Result<Vec<String>, String> {
    const EXTS: &[&str] = &["jpg", "jpeg", "png", "bmp", "tif", "tiff", "webp"];
    let mut out = Vec::new();
    let entries = std::fs::read_dir(dir).map_err(|e| e.to_string())?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if EXTS.contains(&ext.to_lowercase().as_str()) {
                    out.push(path.to_string_lossy().to_string());
                }
            }
        }
    }
    out.sort();
    Ok(out)
}

#[tauri::command]
pub async fn generate_video(
    app: AppHandle,
    config: GenerateConfig,
    state: State<'_, SharedGenerationState>,
) -> Result<(), String> {
    thread::sleep(Duration::from_secs(10));

    let cancelled = {
        let mut gs = state.lock().map_err(|e| e.to_string())?;
        gs.cancelled = Arc::new(AtomicBool::new(false));
        Arc::clone(&gs.cancelled)
    };

    let app_clone = app.clone();
    let cancelled_clone = Arc::clone(&cancelled);

    tokio::task::spawn_blocking(move || {
        let reporter = ProgressReporter {
            callback: Box::new(move |cur, total, pct, msg| {
                let _ = app_clone.emit(
                    "video-progress",
                    ProgressPayload {
                        phase: "encoding".to_string(),
                        current_frame: cur,
                        total_frames: total,
                        percentage: pct,
                        message: msg.to_string(),
                    },
                );
            }),
        };

        run_generation(&config, &reporter, cancelled_clone)
    })
    .await
    .map_err(|e| e.to_string())?
    .map_err(|e| e.to_string())?;

    let _ = app.emit(
        "video-progress",
        ProgressPayload {
            phase: "done".to_string(),
            current_frame: 0,
            total_frames: 0,
            percentage: 100.0,
            message: "Done!".to_string(),
        },
    );

    Ok(())
}

#[tauri::command]
pub fn cancel_generation(state: State<'_, SharedGenerationState>) -> Result<(), String> {
    let gs = state.lock().map_err(|e| e.to_string())?;
    gs.cancelled.store(true, Ordering::SeqCst);
    Ok(())
}
