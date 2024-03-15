


#[cfg(test)]
mod tests {
    use reqwest::Response;
    use crate::{api::Api, config, license::License};

    async fn get_some_db_license() -> License {
        let api = Api::new(config::HOST, config::PORT);
        let res: Response = api.client.get(format!("{}/test_license", api.base_url))
            .send().await.unwrap();
        let content = res.text().await.unwrap();
        let license: License = serde_json::from_str(&content).unwrap();
        return license;
    }
    

    #[tokio::test]
    async fn test_register() {
        let mut api = Api::new("127.0.0.1", &8000);
        let license = get_some_db_license().await;
        let result = api.register(&license.value, "hwid").await;
        if license.hwid.unwrap_or_default() == "" { // not registred already
            assert!(result.is_ok());
        } else {
            assert!(result.is_err())
        }
    }

    #[tokio::test]
    async fn test_revoke() {
        let mut api = Api::new("127.0.0.1", &8000);
        let license = get_some_db_license().await;
        api.revoke(&license.value).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_license() {
        let mut api = Api::new("127.0.0.1", &8000);
        let license = get_some_db_license().await;
        api.get_license(&license.value).await.unwrap();
    }
}