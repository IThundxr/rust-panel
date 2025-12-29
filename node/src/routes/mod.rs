mod api;

use axum::Router;
use crate::app::App;

pub(crate) fn router() -> Router<App> {
    Router::new().nest("/api", api::router())
}