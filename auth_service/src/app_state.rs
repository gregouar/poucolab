use axum::extract::FromRef;

pub use backend_common::db::DbPool;

use crate::data_client::HttpDataClient;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: DbPool,
    pub data_client: HttpDataClient,
}

impl FromRef<AppState> for DbPool {
    fn from_ref(app_state: &AppState) -> DbPool {
        app_state.db_pool.clone()
    }
}

impl FromRef<AppState> for HttpDataClient {
    fn from_ref(app_state: &AppState) -> HttpDataClient {
        app_state.data_client.clone()
    }
}
