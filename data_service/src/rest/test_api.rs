use axum::{extract::State, routing::get, Json, Router};
use chrono::Utc;
use shared::rest::test_api::GetTestDataResponse;

use crate::app_state::AppState;

use backend_common::{db::DbPool, rest::ApiError};

pub fn routes() -> Router<AppState> {
    Router::new().route("/data", get(get_data))
}

async fn get_data(State(db_pool): State<DbPool>) -> Result<Json<GetTestDataResponse>, ApiError> {
    _ = db_pool;
    Ok(Json(GetTestDataResponse {
        processed_at: Utc::now(),
    }))
}
