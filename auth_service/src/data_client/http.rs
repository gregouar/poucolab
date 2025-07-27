use serde::de::DeserializeOwned;
use std::sync::Arc;

use shared::rest::test_api::GetTestDataResponse;

use crate::data_client::DataServiceClient;

#[derive(Debug, Clone)]
pub struct HttpDataClient {
    client: reqwest::Client,
    pub base_url: Arc<String>,
}

impl HttpDataClient {
    pub fn new(base_url: String) -> Self {
        HttpDataClient {
            client: reqwest::Client::new(),
            base_url: Arc::new(base_url),
        }
    }

    async fn get<T>(&self, url: &str) -> Result<T, anyhow::Error>
    where
        T: DeserializeOwned,
    {
        let response = self
            .client
            .get(format!("{}{}", self.base_url, url))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json::<T>().await?)
        } else {
            Err(anyhow::anyhow!("failed to fetch data from {}", url))
        }
    }
}

impl DataServiceClient for HttpDataClient {
    async fn health_check(&self) -> Result<bool, anyhow::Error> {
        Ok(self
            .client
            .get(format!("{}/", self.base_url))
            .send()
            .await?
            .status()
            .is_success())
    }

    async fn get_data(&self) -> Result<GetTestDataResponse, anyhow::Error> {
        self.get("/data").await
    }
}
