use anyhow::Result;
use futures::TryStreamExt;
use reqwest::header;
use serde::de::DeserializeOwned;
use shared::rest::test_api::GetTestDataResponse;
use std::sync::Arc;

use super::StreamResponse;

#[derive(Debug, Clone)]
pub struct DataServiceClient {
    client: reqwest::Client,
    pub base_url: Arc<String>,
}

impl DataServiceClient {
    pub fn new(base_url: String) -> Self {
        DataServiceClient {
            client: reqwest::Client::new(),
            base_url: Arc::new(base_url),
        }
    }

    pub async fn health_check(&self) -> Result<bool> {
        Ok(self
            .client
            .get(format!("{}/", self.base_url))
            .send()
            .await?
            .status()
            .is_success())
    }

    pub async fn get_data(&self) -> Result<GetTestDataResponse> {
        self.get_typed("/data").await
    }

    pub async fn get_data_stream(&self) -> Result<StreamResponse> {
        self.get_stream("/data").await
    }

    // TODO: make generic over method and then create handy shortcuts for GET, POST, etc.

    // For typed responses (normal API calls)
    async fn get_typed<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = self
            .client
            .get(format!("{}{}", self.base_url, path))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(anyhow::anyhow!("failed to fetch data from {}", path))
        }
    }

    // For streaming responses (large datasets)
    async fn get_stream(&self, path: &str) -> Result<StreamResponse> {
        let response = self
            .client
            .get(format!("{}{}", self.base_url, path))
            .send()
            .await?;

        if response.status().is_success() {
            let content_type = response
                .headers()
                .get(header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("application/json")
                .to_string();

            let content_length = response
                .headers()
                .get(header::CONTENT_LENGTH)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse().ok());

            let stream = Box::new(response.bytes_stream().map_err(anyhow::Error::from));

            Ok(StreamResponse {
                stream,
                content_type,
                content_length,
            })
        } else {
            Err(anyhow::anyhow!("failed to fetch data from {}", path))
        }
    }
}
