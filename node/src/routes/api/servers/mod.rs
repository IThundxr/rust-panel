use crate::app::App;
use crate::models::server::Servers;
use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};

pub(crate) fn router() -> Router<App> {
	Router::new().route("/", get(get_servers))
}

async fn get_servers(State(app): State<App>) -> Json<Servers> {
	todo!()
}
