mod app;
mod error;
mod models;
mod routes;

use crate::app::App;
use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use bollard::Docker;
use russh_sftp::protocol::VERSION;
use std::env;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::log::{debug, info};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::filter::LevelFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	debug!("Loading .env");
	dotenvy::dotenv()?;

	debug!("Configuring tracing subscriber from env");
	let env_filter = EnvFilter::builder()
		.with_default_directive(LevelFilter::INFO.into())
		.from_env_lossy();
	tracing_subscriber::fmt().with_env_filter(env_filter).init();

	let app = App::new();

	let router = Router::new()
		.route("/status", get(|| async { StatusCode::OK }))
		.route("/docker/version", get(docker_version))
		.layer(TraceLayer::new_for_http())
		.with_state(app);

	let ip = env::var("APP_IP").unwrap_or("0.0.0.0".to_string());
	let port = env::var("APP_PORT").unwrap_or("3000".to_string());
	let address = format!("{ip}:{port}");

	info!("Listening on {address}");

	let listener = tokio::net::TcpListener::bind(address).await?;
	axum::serve(listener, router).await?;

	Ok(())
}

async fn docker_version(State(app): State<App>) -> impl IntoResponse {
	format!("{:?}", app.docker.version().await.unwrap())
}
