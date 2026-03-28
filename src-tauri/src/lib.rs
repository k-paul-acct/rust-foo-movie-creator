pub mod commands;
pub mod ffmpeg;

use commands::{
    cancel_generation, get_ffmpeg_version, generate_video, list_images_in_dir, SharedGenerationState,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(SharedGenerationState::default())
        .invoke_handler(tauri::generate_handler![
            get_ffmpeg_version,
            list_images_in_dir,
            generate_video,
            cancel_generation,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
