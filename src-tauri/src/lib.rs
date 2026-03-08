use engine_assets::{projects::Project, AssetManager};
use engine_core::paths::init_data_path;
use std::sync::Mutex;

struct LoadedProject {
    project: Project,
    assets: AssetManager,
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
        .invoke_handler(tauri::generate_handler![get_project_names, load_project])
        .plugin(tauri_plugin_opener::init())
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

    let mut current = state.current_project.lock().unwrap();
    *current = Some(LoadedProject { project, assets });

    Ok(())
}
