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
        Ok(self
            .client
            .get(format!("{}/data", self.base_url))
            .send()
            .await?
            .json::<GetTestDataResponse>()
            .await?)
    }
}
