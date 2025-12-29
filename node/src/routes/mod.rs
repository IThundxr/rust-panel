mod api;

use crate::app::App;
use axum::Router;

pub fn router() -> Router<App> {
	Router::new().nest("/api", api::router())
}
