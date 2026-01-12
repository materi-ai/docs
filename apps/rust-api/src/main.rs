use axum::{
    routing::{get, post},
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing::{info, instrument};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use opentelemetry_otlp::WithExportConfig;

#[tokio::main]
async fn main() {
    // 1. Initialize OpenTelemetry Pipeline
    // This sends traces to the Alloy collector defined in our docker-compose
    // Default OTLP gRPC port is 4317. Alloy is reachable at "alloy:4317".
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://alloy:4317"),
        )
        .with_trace_config(
            opentelemetry_sdk::trace::config()
                .with_resource(opentelemetry_sdk::Resource::new(vec![
                    opentelemetry::KeyValue::new("service.name", "atlas-rust-api"),
                    opentelemetry::KeyValue::new("env", "atlas-local"),
                ])),
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)
        .expect("failed to install OpenTelemetry tracer");

    // 2. Create a tracing layer for the registry
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // 3. Initialize subscriber
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info,atlas_rust_api=debug"))
        .with(telemetry)
        .with(tracing_subscriber::fmt::layer()) // Log to stdout as well
        .init();

    // 4. Build application routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/manuscript/sync", post(sync_manuscript));

    // 5. Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[instrument]
async fn health_check() -> &'static str {
    info!("Health check received");
    "OK"
}

#[derive(Deserialize)]
struct ManuscriptPayload {
    project_id: String,
    content: String,
}

#[derive(Serialize)]
struct SyncResponse {
    status: String,
    trace_id: String, // In a real app, we'd extract the trace ID to return it
}

#[instrument(skip(payload))]
async fn sync_manuscript(Json(payload): Json<ManuscriptPayload>) -> Json<SyncResponse> {
    info!("Syncing manuscript for project: {}", payload.project_id);
    
    // Simulate some work
    mock_db_call(&payload.project_id).await;

    Json(SyncResponse {
        status: "synced".to_string(),
        trace_id: "TODO-extract-trace-id".to_string(),
    })
}

#[instrument]
async fn mock_db_call(id: &str) {
    // Simulate database latency
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    info!("Database updated for {}", id);
}
