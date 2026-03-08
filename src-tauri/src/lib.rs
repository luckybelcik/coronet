use engine_assets::{projects::Project, AssetManager};
use engine_core::paths::init_data_path;
use image::EncodableLayout;
use image::ImageFormat;
use std::io::Cursor;
use std::sync::Mutex;
use tauri::Manager;

struct LoadedProject {
    project: Project,
    assets: AssetManager,
    cached_atlas_bytes: Vec<u8>,
}

struct EngineState {
    projects: Vec<Project>,
    current_project: Mutex<Option<LoadedProject>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_data_path().expect("Failed to initialize paths");

    tauri::Builder::default()
        .manage(EngineState {
            projects: engine_assets::projects::Project::find_all(),
            current_project: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            get_project_names,
            load_project,
            get_block_names,
            get_blocks_with_preview
        ])
        .plugin(tauri_plugin_opener::init())
        .register_uri_scheme_protocol("engine-asset", move |app, request| {
            let uri = request.uri();
            let path = uri.path();

            println!("Full URI: {:?}", uri);
            println!("Path: {:?}", path);

            let state = app.app_handle().state::<EngineState>();
            let current_lock = state.current_project.lock().unwrap();

            if path.ends_with("block_atlas.png") {
                if let Some(loaded_project) = current_lock.as_ref() {
                    return tauri::http::Response::builder()
                        .header("Content-Type", "image/png")
                        .status(200)
                        .body(loaded_project.cached_atlas_bytes.clone())
                        .unwrap();
                }
            }

            tauri::http::Response::builder()
                .status(404)
                .body(Vec::new())
                .unwrap()
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_project_names(state: tauri::State<'_, EngineState>) -> Vec<String> {
    state.projects.iter().map(|p| p.name.clone()).collect()
}

#[tauri::command]
fn load_project(state: tauri::State<'_, EngineState>, project_name: String) -> Result<(), String> {
    let project = state
        .projects
        .iter()
        .find(|p| p.name == project_name)
        .cloned()
        .ok_or_else(|| "Project not found".to_string())?;

    let assets = AssetManager::init(Some(vec![project.name.clone()]), false);

    let mut buffer = Cursor::new(Vec::new());

    assets
        .block_atlas
        .write_to(&mut buffer, ImageFormat::Png)
        .map_err(|e| format!("Failed to encode atlas: {}", e))?;

    let cached_atlas_bytes = buffer.into_inner();

    let mut current = state.current_project.lock().unwrap();
    *current = Some(LoadedProject {
        project,
        assets,
        cached_atlas_bytes,
    });

    Ok(())
}

#[tauri::command]
fn get_block_names(state: tauri::State<'_, EngineState>) -> Vec<String> {
    let mut current = state.current_project.lock().unwrap();
    if let Some(loaded_project) = current.as_mut() {
        let blocks = loaded_project.assets.block_registry.get_all_blocks();
        blocks.iter().map(|b| b.0.clone()).collect()
    } else {
        Vec::new()
    }
}

#[tauri::command]
fn get_blocks_with_preview(state: tauri::State<'_, EngineState>) -> Vec<(String, u16, u32, u32)> {
    let current = &state.current_project.lock().unwrap();
    if let Some(loaded_project) = current.as_ref() {
        let assets = &loaded_project.assets;
        let atlas_dim = (assets.block_atlas.width() / 16) as u32;

        assets
            .block_registry
            .get_all_blocks()
            .iter()
            .map(|(name, id)| {
                let face_index = ((*id + 1) * 6) + 2;
                let atlas_index = assets.texture_mapping_table[face_index as usize];

                let u = (atlas_index % atlas_dim) * 16;
                let v = (atlas_index / atlas_dim) * 16;

                ((*name).clone(), (**id + 1), u, v)
            })
            .collect()
    } else {
        Vec::new()
    }
}
