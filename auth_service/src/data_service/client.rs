use anyhow::Result;
use futures::TryStreamExt;
use http::Method;
use reqwest::{header, Body};
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

    // For typed responses (normal API calls)
    async fn get_typed<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        Ok(self.get(path).await?.json().await?)
    }

    // For streaming responses (large datasets)
    async fn get_stream(&self, path: &str) -> Result<StreamResponse> {
        let response = self.get(path).await?;

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
    }

    async fn get(&self, path: &str) -> Result<reqwest::Response> {
        self.request(Method::GET, path, None).await
    }

    // async fn post(&self, path: &str, body: Body) -> Result<reqwest::Response> {
    //     self.request(Method::GET, path, Some(body)).await
    // }

    async fn request(
        &self,
        method: Method,
        path: &str,
        body: Option<Body>,
    ) -> Result<reqwest::Response> {
        let mut request = self
            .client
            .request(method, format!("{}{}", self.base_url, path));

        if let Some(body) = body {
            request = request.body(body);
        }

        let response = request.send().await?;

        if response.status().is_success() {
            Ok(response)
        } else {
            Err(anyhow::anyhow!(
                "failed to contact data service on {}",
                path
            ))
        }
    }
}
