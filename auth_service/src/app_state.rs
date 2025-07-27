use axum::extract::FromRef;

pub use backend_common::db::DbPool;

use crate::data_service::DataServiceClient;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: DbPool,
    pub data_service_client: DataServiceClient,
}

impl FromRef<AppState> for DbPool {
    fn from_ref(app_state: &AppState) -> DbPool {
        app_state.db_pool.clone()
    }
}

impl FromRef<AppState> for DataServiceClient {
    fn from_ref(app_state: &AppState) -> DataServiceClient {
        app_state.data_service_client.clone()
    }
}
