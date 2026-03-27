use engine_assets::{projects::Project, AssetManager};
use engine_core::paths::init_data_path;
use image::{DynamicImage, GenericImage, GenericImageView, ImageFormat, Rgba};
use std::io::Cursor;
use std::sync::Mutex;
use tauri::Manager;

struct LoadedProject {
    project: Project,
    assets: AssetManager,
    rendered_images: Vec<DynamicImage>,
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

            let state = app.app_handle().state::<EngineState>();
            let current_lock = state.current_project.lock().unwrap();

            if let Some(loaded_project) = current_lock.as_ref() {
                if uri.to_string().starts_with("engine-asset://blocks/") {
                    let index = path
                        .split('/')
                        .last()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap_or(0);
                    if let Some(img) = loaded_project.rendered_images.get(index) {
                        let mut buffer = Cursor::new(Vec::new());
                        if img.write_to(&mut buffer, ImageFormat::Png).is_ok() {
                            return tauri::http::Response::builder()
                                .header("Content-Type", "image/png")
                                .status(200)
                                .body(buffer.into_inner())
                                .unwrap();
                        }
                    }
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

    let assets: AssetManager = AssetManager::init(Some(vec![project.name.clone()]), false, true)?.0;

    let mut blocks = assets.block_registry.get_all_blocks();
    blocks.sort_by_key(|block| *block.1); // sort by block id

    let mut rendered_images = Vec::new();

    for block in blocks {
        if *block.1 == 0 {
            continue;
        }

        let image = render_block_image(&assets, *block.1);
        rendered_images.push(image);
    }

    let mut current = state.current_project.lock().unwrap();
    *current = Some(LoadedProject {
        project,
        assets,
        rendered_images,
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
fn get_blocks_with_preview(state: tauri::State<'_, EngineState>) -> Vec<(String, u16, u32)> {
    let current = &state.current_project.lock().unwrap();
    if let Some(loaded_project) = current.as_ref() {
        let assets = &loaded_project.assets;

        let mut blocks = assets.block_registry.get_all_blocks();
        blocks.sort_by_key(|block| *block.1); // sort by block id

        blocks[1..] // exclude air by slicing it out
            .iter()
            .map(|(name, id)| {
                let face_index = (*id) * 6;
                let face_metadata = assets.metadata_table[(face_index - 6) as usize];
                let atlas_index;
                // variant bit
                if face_metadata.additional_meta & 1 == 0 {
                    atlas_index = assets.texture_mapping_table[face_index as usize];
                } else {
                    let variant_data = assets.texture_mapping_table[face_index as usize];
                    let offset = variant_data & 0x0FFFFFFF;
                    atlas_index = assets.texture_variant_mapping_table[offset as usize];
                }

                ((*name).clone(), (**id), atlas_index)
            })
            .collect()
    } else {
        Vec::new()
    }
}

fn render_block_image(assets: &AssetManager, block_id: u16) -> DynamicImage {
    let face_index = block_id * 6;
    let face_metadata = assets.metadata_table[(face_index - 6) as usize];
    let atlas_index;
    // block variant bit
    if face_metadata.additional_meta & 1 == 0 {
        atlas_index = assets.texture_mapping_table[face_index as usize];
    } else {
        let variant_data = assets.texture_mapping_table[face_index as usize];
        let offset = variant_data & 0x0FFFFFFF;
        atlas_index = assets.texture_variant_mapping_table[offset as usize];
    }

    let mask_index;
    // mask variant bit
    if face_metadata.additional_meta & 2 == 0 {
        mask_index = face_metadata.mask_atlas_id;
    } else {
        let variant_data = face_metadata.mask_atlas_id;
        let offset = variant_data & 0x0FFFFFFF;
        mask_index = assets.colormap_mask_variant_mapping_table[offset as usize];
    }

    let mut image;
    if let Some(blocks) = &assets.active_block_textures {
        image = image::load_from_memory_with_format(&blocks[atlas_index as usize], ImageFormat::Qoi).unwrap(); // QOI format here
    } else {
        image = DynamicImage::new_rgb8(16, 16);
    }

    if let Some(masks) = &assets.active_colormap_masks_textures {
        if mask_index > 0 {
            if let Some(colormaps) = &assets.active_colormap_textures {
                let mask: DynamicImage = image::load_from_memory_with_format(&masks[mask_index.saturating_sub(1) as usize], ImageFormat::Png).unwrap(); // PNG format here bc its grayscale
                let colormap_0_id = face_metadata.packed_colormap_ids & 0x7FF;
                let colormap_1_id = (face_metadata.packed_colormap_ids >> 11) & 0x7FF;
                let colormap_2_id = (face_metadata.packed_colormap_ids >> 22) & 0x3FF;
                // colormaps in QOI
                let colormap_0: Option<DynamicImage> = if let Some(bytes) = &colormaps.get(colormap_0_id.saturating_sub(1) as usize) { image::load_from_memory_with_format(bytes, ImageFormat::Qoi).ok() } else { None };
                let colormap_1: Option<DynamicImage> = if let Some(bytes) = &colormaps.get(colormap_1_id.saturating_sub(1) as usize) { image::load_from_memory_with_format(bytes, ImageFormat::Qoi).ok() } else { None };
                let colormap_2: Option<DynamicImage> = if let Some(bytes) = &colormaps.get(colormap_2_id.saturating_sub(1) as usize) { image::load_from_memory_with_format(bytes, ImageFormat::Qoi).ok() } else { None };
                for y in 0..16 {
                    for x in 0..16 {
                        let mask_pixel = mask.get_pixel(x, y);
                        let weight_0 = (((mask_pixel[0] as u32 & 7) * 255) / 7) as u8;
                        let weight_1 = ((((mask_pixel[0] >> 3) as u32 & 7) * 255) / 7) as u8;
                        let weight_2 = ((((mask_pixel[0] >> 6) as u32 & 3) * 255) / 3) as u8;

                        let mut color = image.get_pixel(x, y);
                        if let Some(colormap_0) = &colormap_0 {
                            let second_color = multiply_colors_u8(
                                color,
                                colormap_0.get_pixel(((x + 1) * 8) - 1, (y + 1 * 8) - 1),
                            );
                            color = mix_colors_u8(color, second_color, weight_0)
                        }
                        if let Some(colormap_1) = &colormap_1 {
                            let second_color = multiply_colors_u8(
                                color,
                                colormap_1.get_pixel(((x + 1) * 8) - 1, (y + 1 * 8) - 1),
                            );
                            color = mix_colors_u8(color, second_color, weight_1)
                        }
                        if let Some(colormap_2) = &colormap_2 {
                            let second_color = multiply_colors_u8(
                                color,
                                colormap_2.get_pixel(((x + 1) * 8) - 1, (y + 1 * 8) - 1),
                            );
                            color = mix_colors_u8(color, second_color, weight_2)
                        }
                        image.put_pixel(x, y, color);
                    }
                }
            }
        }
    }

    image
}

fn multiply_colors_u8(c1: Rgba<u8>, c2: Rgba<u8>) -> Rgba<u8> {
    Rgba::<u8>::from([
        ((c1[0] as u32 * c2[0] as u32) / 255) as u8,
        ((c1[1] as u32 * c2[1] as u32) / 255) as u8,
        ((c1[2] as u32 * c2[2] as u32) / 255) as u8,
        ((c1[3] as u32 * c2[3] as u32) / 255) as u8,
    ])
}

fn mix_colors_u8(c1: Rgba<u8>, c2: Rgba<u8>, mask: u8) -> Rgba<u8> {
    let m = mask as u32;
    let inv_m = 255 - m;

    Rgba::<u8>::from([
        ((c1[0] as u32 * inv_m + c2[0] as u32 * m) / 255) as u8,
        ((c1[1] as u32 * inv_m + c2[1] as u32 * m) / 255) as u8,
        ((c1[2] as u32 * inv_m + c2[2] as u32 * m) / 255) as u8,
        ((c1[3] as u32 * inv_m + c2[3] as u32 * m) / 255) as u8,
    ])
}
