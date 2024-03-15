

use std::fs;
use anyhow::Result;
use tauri;
use machineid_rs::HWIDComponent;
use machineid_rs::{IdBuilder, Encryption};
use crate::api::Api;
use crate::config;
use super::internet;
use crate::license::License;
use log::{debug,info, error};

const INTERNET_CHECK_TIMEOUT: &u64 = &1;

pub struct LicenseManager {
    api: Api
}

impl LicenseManager {


    pub fn new(api: Api) -> Self {
        return LicenseManager { api }
    }
    
    pub fn create_system_id(&self, app: &tauri::AppHandle) -> String {
        let package_info = app.package_info();
        let mut builder = IdBuilder::new(Encryption::SHA256);
        builder.add_component(HWIDComponent::SystemID);
        
        let hwid: String = builder.build(&package_info.name).unwrap();
        hwid
    }

    fn local_valid(&mut self, app: &tauri::AppHandle) -> Result<bool> {
        let data_path = app.path_resolver().app_data_dir().unwrap();
        let license_path = data_path.join(config::LICENSE_FILE_NAME);
    
        if !license_path.exists() {
            return Ok(false);
        }
        let license = self.load(app)?;
        // compare this PC unique ID with the license registred ID
        let sys_id = self.create_system_id(app);
        if sys_id != license.hwid.to_owned().unwrap_or_default() {
            return Ok(false)
        }
        // Compre expire time to current timestamp
        if license.is_expired() {
            error!("license expired!");
            return Ok(false)
        }
        Ok(true)
    }

    async fn refresh(&mut self, app: &tauri::AppHandle) -> Result<()> {
        let license = self.load(app)?;
        let new_license = self.api.get_license(&license.value).await?;
        self.save(app, &new_license)?;
        Ok(())
    }
    
    pub async fn is_valid(&mut self, app: &tauri::AppHandle) -> Result<bool> {
        
        // 1. Do we have it in file?
        // 2. Do we have internet connection?
        // 3. Does the server available?
        // 4. Is the license active?
        // 5. Is the mac of this computer same as the registred for this license?

        if !self.local_valid(&app)? {
            return Ok(false);
        }
        // check intenret...
        if !internet::online(INTERNET_CHECK_TIMEOUT).await {
            debug!("No internet... so license considered valid");
            return Ok(true);
        } else {
            debug!("detected internet connection");
        }
        match self.refresh(app).await {
            Ok(_) => {
                // check refreshed one
                let valid = self.local_valid(&app)?;
                debug!("License valid: {valid}");
                return Ok(valid);
            },
            Err(_) => {
                info!("couldnt reach server for validate, assuming valid license");
                return Ok(true)
            }
        };
        
    }


    pub fn load(&self, app: &tauri::AppHandle) -> Result<License> {
        let data_path = app.path_resolver().app_data_dir().unwrap();
        let license_path = data_path.join(config::LICENSE_FILE_NAME);
        let license_data = fs::read_to_string(license_path)?;
        let license: License = serde_json::from_str(&license_data)?;
        Ok(license)
    }

    pub fn save(&self, app: &tauri::AppHandle, license: &License) -> Result<()> {
        // possible improvments: can store encrypted
        let data_path = app.path_resolver().app_data_dir().unwrap();
        let license_path = data_path.join(config::LICENSE_FILE_NAME);
        debug!("saving in {license_path:?}");
        fs::write(license_path, serde_json::to_string_pretty(&license)?)?;
        Ok(())
    }

    pub fn delete_license(&self, app: &tauri::AppHandle) -> Result<()> {
        let data_path = app.path_resolver().app_data_dir().unwrap();
        let license_path = data_path.join(config::LICENSE_FILE_NAME);
        fs::remove_file(license_path)?;
        Ok(())
    }
}

