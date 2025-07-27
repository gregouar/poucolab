use axum::extract::FromRef;

use backend_common::db::DbPool;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: DbPool,
}

impl FromRef<AppState> for DbPool {
    fn from_ref(app_state: &AppState) -> DbPool {
        app_state.db_pool.clone()
    }
}
