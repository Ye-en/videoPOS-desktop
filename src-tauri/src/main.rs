// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod config;
mod license;
use std::fs;
use api::Api;
use license::manager::LicenseManager;
use tauri::Manager;
mod cmd;
use env_logger;
use auto_launch::AutoLaunch;



fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {

    // can do more checks here for license
    let data_path = app.path_resolver().app_data_dir().unwrap();
    fs::create_dir_all(data_path).unwrap();
    let main_window = app.get_window("main").unwrap();
    // main_window.eval(&format!("window.location.href = '/setup'"));
    main_window.show()?;
    let api = Api::new(config::HOST, config::PORT);
    let mut manager = LicenseManager::new(api);
    let valid: bool = tauri::async_runtime::block_on(async {
        let valid = manager.is_valid(&app.app_handle()).await.unwrap();
        return valid;
    });
    if !valid {
        main_window.eval(&format!("window.location.href = '/setup'")).unwrap();
    } else {
        main_window.eval(&format!("window.location.href = '/home'")).unwrap();
    }
    Ok(())
}

fn main() {
    let app_name = "VideoPOS";
    let app_path = "C:\\path\\to\\the-app.exe";
    let args = &["--minimized"];
    let auto = AutoLaunch::new(app_name, app_path, true, args);

    // enable the auto launch
    auto.enable().is_ok();
    auto.is_enabled().unwrap();

    env_logger::try_init().unwrap();
    tauri::Builder::default()
        .setup(setup)
        .invoke_handler(tauri::generate_handler![cmd::is_valid, cmd::register, cmd::revoke, cmd::get_license])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
