#![forbid(unsafe_code)]
#![warn(clippy::unwrap_used)] // TODO - Change to forbid later

mod app;
mod config;
mod error;
mod models;
mod routes;

use crate::app::App;
use crate::config::Config;
use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use std::env;
use tower_http::trace::TraceLayer;
use tracing::level_filters::LevelFilter;
use tracing::log::{debug, info};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	debug!("Loading .env");
	dotenvy::dotenv()?;

	debug!("Configuring tracing subscriber from env");
	let env_filter = EnvFilter::builder()
		.with_default_directive(LevelFilter::INFO.into())
		.from_env_lossy();
	tracing_subscriber::fmt().with_env_filter(env_filter).init();

	let _ = Config::setup_watcher();
	let app = App::new();

	let router = Router::new()
		.route("/status", get(StatusCode::OK))
		.route("/docker/version", get(docker_version))
		.merge(routes::router())
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
