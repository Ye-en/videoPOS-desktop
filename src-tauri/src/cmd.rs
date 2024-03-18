use crate::api::Api;
use crate::config;
use crate::license::License;
use tauri::Manager;
use crate::license::manager::LicenseManager;

#[tauri::command]
pub async fn is_valid(app: tauri::AppHandle) -> Result<bool, String> {
    let api = Api::new(config::HOST, config::PORT);
    let mut manager = LicenseManager::new(api);
    let valid = manager.is_valid(&app).await.unwrap();
    Ok(valid)
}

#[tauri::command]
pub async fn register(app: tauri::AppHandle, value: String) -> Result<(), String> {
    let mut api = Api::new(config::HOST, config::PORT);
    let manager = LicenseManager::new(api.clone());
    let license = api.register(&value, &manager.create_system_id(&app)).await.map_err(|e| e.to_string())?;
    
    let main_window = app.get_window("main").unwrap();
    main_window.eval(&format!("window.location.href = '/home'")).unwrap();
    manager.save(&app, &license).map_err(|e| e.to_string())?;
    Ok(())
}


#[tauri::command]
pub async fn revoke(app: tauri::AppHandle) -> Result<(), String> {
    let mut api = Api::new(config::HOST, config::PORT);
    let manager = LicenseManager::new(api.clone());
    let license = manager.load(&app).map_err(|e| e.to_string())?;
    api.revoke(&license.value).await.map_err(|e| e.to_string())?;
    manager.delete_license(&app).map_err(|e| e.to_string())?;
    let main_window = app.get_window("main").unwrap();
    main_window.eval(&format!("window.location.href = '/setup'")).unwrap();
    Ok(())
}

#[tauri::command]
pub async fn get_license(app: tauri::AppHandle) -> Result<License, String> {
    let api = Api::new(config::HOST, config::PORT);
    let manager = LicenseManager::new(api.clone());
    let license = manager.load(&app).map_err(|e| e.to_string())?;
    
    Ok(license)
}

use quick_xml::{Reader, Writer, events::{Event, BytesEnd, BytesStart, BytesText}};
use std::fs::{self, File};
use std::io::{Cursor, BufReader};
use std::path::PathBuf;

#[derive(serde::Deserialize)]
struct Config {
    server_ip: String,
    stream_uri: String,
    fps: i32,
    encoding: String,
    dimensions: String,
}

#[tauri::command]
fn set_config(new_config: Config, app_handle: tauri::AppHandle) -> Result<(), String> {
    let mut config_path = std::env::current_exe().map_err(|e| e.to_string())?;
    config_path.pop(); // Remove the executable name
    config_path.push("binaries/config.xml"); // Add the relative path to the config file

    let file = File::open(config_path).map_err(|e| e.to_string())?;
    let file_buffer = BufReader::new(file);
    let mut reader = Reader::from_reader(file_buffer);
    let mut writer = Writer::new(Cursor::new(Vec::new()));

    let mut buf = Vec::new();
    let mut txt = Vec::new();
    let mut in_video_encoder = false;

    // Parse dimensions
    let dimensions: Vec<&str> = new_config.dimensions.split('x').collect();
    let (width, height) = if dimensions.len() == 2 {
        (dimensions[0], dimensions[1])
    } else {
        ("0", "0")
    };

    loop {
        match reader.read_event_into(&mut buf, &mut txt) {
            Ok(Event::Start(ref e)) => {
                if e.name() == b"video_encoder" {
                    in_video_encoder = true;
                }
                writer.write_event(Event::Start(e.clone())).unwrap();
            }
            Ok(Event::End(ref e)) => {
                if e.name() == b"video_encoder" {
                    in_video_encoder = false;
                }
                writer.write_event(Event::End(e.clone())).unwrap();
            }
            Ok(Event::Text(e)) if in_video_encoder => {
                let current_tag = reader.tag_name().expect("Unable to get tag name");

                let new_text = match current_tag {
                    b"framerate" => new_config.fps.to_string(),
                    b"encoding" => new_config.encoding.clone(),
                    b"width" if reader.has_ancestor(b"video_encoder") => width.to_string(),
                    b"height" if reader.has_ancestor(b"video_encoder") => height.to_string(),
                    _ => e.unescape_and_decode(&reader).unwrap(),
                };

                writer.write_event(Event::Text(BytesText::from_plain_str(&new_text))).unwrap();
            }
            Ok(Event::Text(e)) => {
                // For nodes outside video_encoder, keep the text as is
                writer.write_event(Event::Text(e.clone())).unwrap();
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("Error reading XML: {}", e)),
            _ => (),
        }
        buf.clear();
    }

    // Write the updated XML back to the file
    let result = writer.into_inner().into_inner();
    fs::write(config_path, &result).map_err(|e| e.to_string())?;

    Ok(())
}
