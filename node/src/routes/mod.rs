mod api;

use crate::app::App;
use axum::Router;

pub(crate) fn router() -> Router<App> {
	Router::new().nest("/api", api::router())
}
