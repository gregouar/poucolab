use axum::Router;

use crate::app_state::AppState;

mod test_api;

pub fn routes() -> Router<AppState> {
    Router::new().merge(test_api::routes())
}
