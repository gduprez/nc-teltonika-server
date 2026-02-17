use axum::{
    routing::get,
    Router,
    response::{IntoResponse, Response},
    http::StatusCode,
};
use std::net::SocketAddr;
use metrics_exporter_prometheus::PrometheusBuilder;
use tracing::info;
use sqlx::PgPool;

use crate::db::TeltonikaDataRepo;

pub async fn start(port: u16, db_pool: PgPool) {
    let builder = PrometheusBuilder::new();
    let recorder_handle = builder.install_recorder()
        .expect("failed to install Prometheus recorder");

    let app = Router::new()
        .route("/health", get(move || health_handler(db_pool)))
        .route("/metrics", get(move || std::future::ready(recorder_handle.render())));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Helper HTTP server (health/metrics) listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_handler(pool: PgPool) -> Response {
    match TeltonikaDataRepo::check_health(&pool).await {
        Ok(_) => (StatusCode::OK, "OK").into_response(),
        Err(e) => {
            tracing::error!("Health check failed: {}", e);
            (StatusCode::SERVICE_UNAVAILABLE, "DB connection failed").into_response()
        }
    }
}
