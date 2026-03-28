use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};
use tokio_util::sync::CancellationToken;

use crate::ffmpeg::{ProgressReporter, ffmpeg_version, run_generation};

pub struct GenerationState {
    pub ct: CancellationToken,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ScreensaverShapeColor {
    pub color: String,
    pub alpha: u32,
}

#[derive(Debug, Deserialize)]
pub struct ScreensaverConfig {
    pub shape_type: String,
    pub shape_count: u32,
    pub min_size: u32,
    pub max_size: u32,
    pub min_speed: u32,
    pub max_speed: u32,
    pub bg_color: String,
    pub colors: Vec<ScreensaverShapeColor>,
}

#[derive(Debug, Deserialize)]
pub struct GenerateConfig {
    pub output_path: String,
    pub codec: String,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub quality: u32,
    pub duration: f32,
    pub seed: Option<u32>,
    // pub transition: String,
    // pub images: Vec<String>,
    // pub effects: Vec<String>,
    // pub min_dur: f32,
    // pub max_dur: f32,
    // pub total_dur: Option<f32>,
    // pub seed: Option<u32>,
    // pub no_repeat: bool,
    pub screensaver: Option<ScreensaverConfig>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProgressPayload {
    pub phase: String,
    pub percentage: u32,
    pub message: String,
}

pub type SharedGenerationState = Mutex<GenerationState>;

impl Default for GenerationState {
    fn default() -> Self {
        Self {
            ct: CancellationToken::new(),
        }
    }
}

#[tauri::command]
pub async fn get_ffmpeg_version() -> Result<String, String> {
    ffmpeg_version().await.map_err(|e| e.to_string())
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
    let ct = {
        let gs = state.lock().map_err(|e| e.to_string())?;
        gs.ct.clone()
    };

    let mut reporter = ProgressReporter::from_callback(Box::new(move |stage, pct, msg| {
        _ = app.emit(
            "video-progress",
            ProgressPayload {
                phase: stage.to_string(),
                percentage: pct,
                message: msg.to_string(),
            },
        );
    }));

    run_generation(&config, &mut reporter, ct)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn cancel_generation(state: State<'_, SharedGenerationState>) -> Result<(), String> {
    let gs = state.lock().map_err(|e| e.to_string())?;
    gs.ct.cancel();
    Ok(())
}
