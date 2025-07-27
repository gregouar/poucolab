use shared::rest::test_api::GetTestDataResponse;

#[allow(async_fn_in_trait)]
pub trait DataServiceClient: Send + Sync {
    async fn health_check(&self) -> Result<bool, anyhow::Error>;
    async fn get_data(&self) -> Result<GetTestDataResponse, anyhow::Error>;
}
