use std::net::SocketAddr;

use axum::{routing::get, Router};
use http::{HeaderValue, Method};
use tokio::signal;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use auth_service::{data_client::HttpDataClient, rest, AppState};
use backend_common::db::pool;

#[tokio::main]
async fn main() {
    let _ = dotenvy::from_path("auth_service/.env");

    // TODO: depending on environment, only install necessary
    sqlx::any::install_default_drivers();

    let db_pool =
        pool::create_pool(&std::env::var("DATABASE_URL").expect("missing 'DATABASE_URL' setting"))
            .await
            .expect("failed to connect to database");

    pool::migrate(&db_pool, "auth_service/migrations")
        .await
        .expect("failed to migrate database");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let tracer_layer =
        TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default().include_headers(true));

    let cors_layer = CorsLayer::new()
        .allow_origin([std::env::var("CORS_ORIGIN")
            .expect("missing 'CORS_ORIGIN' setting")
            .parse::<HeaderValue>()
            .unwrap()])
        .allow_methods([Method::GET, Method::POST]);

    let data_client = HttpDataClient::new(
        std::env::var("DATA_SERVICE_URL").expect("missing 'DATA_SERVICE_URL' setting"),
    );

    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .merge(rest::routes())
        .with_state(AppState {
            db_pool,
            data_client,
        })
        .layer(tracer_layer)
        .layer(cors_layer);

    let listener = tokio::net::TcpListener::bind(format!(
        "0.0.0.0:{}",
        std::env::var("APP_PORT").expect("missing 'APP_PORT' setting")
    ))
    .await
    .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    let server = axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    );

    tokio::select! {
        _ = server => {},
        _ = signal::ctrl_c() => {
            tracing::debug!("received shutdown signal");
        }
    }
}
