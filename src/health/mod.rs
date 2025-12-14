use axum::Router;
use crate::AppState;

pub mod health_check;

pub fn router() -> Router<AppState> {
    Router::new().route("/", axum::routing::get(health_check::health_check))
}
