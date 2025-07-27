use axum::{extract::State, routing::get, Json, Router};
use chrono::Utc;
use shared::rest::test_api::{GetTestDataResponse, GetTestResponse};

use crate::{
    app_state::AppState,
    data_client::{DataServiceClient, HttpDataClient},
};

use backend_common::{db::DbPool, rest::ApiError};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/test", get(get_test))
        .route("/data", get(get_data))
}

async fn get_test(State(db_pool): State<DbPool>) -> Result<Json<GetTestResponse>, ApiError> {
    _ = db_pool;
    Ok(Json(GetTestResponse { now: Utc::now() }))
}

async fn get_data(
    State(data_client): State<HttpDataClient>,
) -> Result<Json<GetTestDataResponse>, ApiError> {
    Ok(Json(data_client.get_data().await?))
}
