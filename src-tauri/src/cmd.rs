use crate::api::Api;
use crate::config;
use crate::license::License;
use tauri::Manager;
use crate::license::manager::LicenseManager;
use tauri::api::process::Command;

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



#[tauri::command]
pub fn run_onvif_server(app: tauri::AppHandle) -> Result<(), String> {
        // `new_sidecar()` expects just the filename, NOT the whole path like in JavaScript
    let (mut rx, mut child) = Command::new_sidecar("OnvifServer")
    .expect("failed to create `sidecar` binary command")
    .spawn()
    .expect("Failed to spawn sidecar");

    tauri::async_runtime::spawn(async move {
    // read events such as stdout
    while let Some(event) = rx.recv().await {
    if let CommandEvent::Stdout(line) = event {
        window
        .emit("message", Some(format!("'{}'", line)))
        .expect("failed to emit event");
        // write to stdin
        child.write("message from Rust\n".as_bytes()).unwrap();
    }
    }
    });
    Ok(())
}



// Tauri command for streaming address
use get_if_addrs::get_if_addrs;

#[tauri::command]
pub fn address(address: &str) -> String {
    format!("Currently streaming on IP Address: {}", address)
}


#[tauri::command]
pub fn get_local_ip() -> Result<String, String> {
    let addrs = get_if_addrs().map_err(|e| e.to_string())?;
    for iface in addrs {
        if !iface.is_loopback() && iface.ip().is_ipv4() {
            // Returning the first non-loopback IPv4 address
            return Ok(iface.ip().to_string());
        }
    }
    Err("No suitable IP address found".to_string())
}

