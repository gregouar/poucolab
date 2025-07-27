use axum::{extract::State, response::Response, routing::get, Json, Router};
use chrono::Utc;
use shared::rest::test_api::{GetTestDataResponse, GetTestResponse};

use crate::{
    app_state::AppState,
    data_service::{stream_response, DataServiceClient},
};

use backend_common::{db::DbPool, rest::ApiError};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/test", get(get_test))
        .route("/data", get(get_data))
        .route("/data_stream", get(get_stream))
}

async fn get_test(State(db_pool): State<DbPool>) -> Result<Json<GetTestResponse>, ApiError> {
    _ = db_pool;
    Ok(Json(GetTestResponse { now: Utc::now() }))
}

async fn get_data(
    State(data_service_client): State<DataServiceClient>,
) -> Result<Json<GetTestDataResponse>, ApiError> {
    Ok(Json(data_service_client.get_data().await?))
}

async fn get_stream(
    State(data_service_client): State<DataServiceClient>,
) -> Result<Response, ApiError> {
    Ok(stream_response(
        data_service_client.get_data_stream().await?,
    )?)
}
