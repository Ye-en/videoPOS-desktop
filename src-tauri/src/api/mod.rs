use std::time::Duration;

use reqwest::{Client, Response};
use anyhow::{Result, anyhow, Error};
use serde_json::{self, Value};
mod test;
use crate::license::License;
use log::debug;

#[derive(Clone)]
pub struct Api {
    base_url: String,
    client: Client
}


impl Api {
    pub fn new(host: &str, port: &i32) -> Self {
        let client = Client::builder().timeout(Duration::from_secs(3)).build().unwrap();
        Api { base_url: format!("https://{host}/api"), client }
    }

    pub async fn register(&mut self, value: &str, hwid: &str) -> Result<License, Error> {
        let body = serde_json::json!({"value": value, "hwid": hwid});
        debug!("Registering...");
        let res: reqwest::Response = self.client.post(format!("{}/register", self.base_url))
            .json(&body)
            .send().await?;
        debug!("Done...");
        if res.status().as_u16() != 200 {
            let info: String = res.text().await?;
            let info: Value = serde_json::from_str(&info)?;
            let msg = info["error"].as_str().unwrap_or("Activation failed with unknown error");
            return Err(anyhow!(msg.to_owned()));
        }
        let license: License = res.json().await?;
        Ok(license)
    }

    pub async fn revoke(&mut self, value: &str) -> Result<()> {
        let body = serde_json::json!({"value": value});
        self.client.post(format!("{}/revoke", self.base_url))
            .json(&body)
            .send().await?;
        Ok(())
    }

    pub async fn get_license(&mut self, value: &str) -> Result<License> {
        let body = serde_json::json!({"value": value});
        let res: Response = self.client.post(format!("{}/get_license", self.base_url))
            .json(&body)
            .send().await?;
        let content = res.text().await?;
        debug!("json license is {content}");
        let license: License = serde_json::from_str(&content)?;
        Ok(license)
    }



}