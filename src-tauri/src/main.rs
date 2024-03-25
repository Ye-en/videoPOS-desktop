// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod config;
mod license;
use std::fs;
use api::Api;
use license::manager::LicenseManager;
use tauri::{Manager, WindowEvent};

use std::sync::{Arc, Mutex};
mod cmd;
use env_logger;
use auto_launch::AutoLaunch;



#[cfg(target_os = "macos")]
fn setup_auto_launch() {
    let app_name = "VideoPOS";
    let app_path = std::env::current_exe().expect("Failed to get current executable path").to_str().unwrap().to_string();
    let use_launch_agent = false; // Set to false if you prefer AppleScript

    let auto = AutoLaunch::new(app_name, &app_path, use_launch_agent, &[] as &[&str]);

    // Enable or disable based on your logic
    if !auto.is_enabled().unwrap() {
        auto.enable().unwrap();
    }
}

#[cfg(target_os = "windows")]
fn setup_auto_launch() {
    // Auto-launch logic
    let app_name = "VideoPOS";
    let app_path = std::env::current_exe().expect("Failed to get current executable path").to_str().unwrap().to_string();

    let auto = AutoLaunch::new(app_name, app_path, &[] as &[&str]);
    
    // Enable or disable based on your logic
    if !auto.is_enabled().unwrap() {
        auto.enable().unwrap();
    }
}




fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let data_path = app.path_resolver().app_data_dir().unwrap();
    fs::create_dir_all(data_path).unwrap();

    let main_window = Arc::new(Mutex::new(app.get_window("main").unwrap()));
    let main_window_clone = main_window.clone();

    main_window.lock().unwrap().show()?;

    main_window.lock().unwrap().on_window_event(move |event| {
        match event {
            WindowEvent::CloseRequested { api, .. } => {
                // Prevent the window from closing
                api.prevent_close();
                // Optionally, hide the window to keep the app running in the background
                let _ = main_window_clone.lock().unwrap().hide();
            }
            _ => {}
        }
    });
    setup_auto_launch();


    let api = Api::new(config::HOST, config::PORT);
    let mut manager = LicenseManager::new(api);
    let valid: bool = tauri::async_runtime::block_on(async {
        manager.is_valid(&app.app_handle()).await.unwrap()
    });

    if !valid {
        main_window.lock().unwrap().eval("window.location.href = '/setup'").unwrap();
    } else {
        main_window.lock().unwrap().eval("window.location.href = '/home'").unwrap();

        
    }

    Ok(())
}

fn main() {
    env_logger::try_init().unwrap();
    tauri::Builder::default()
        .setup(setup)
        .invoke_handler(tauri::generate_handler![cmd::is_valid, cmd::register, cmd::revoke, cmd::get_license, cmd::run_onvif_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
