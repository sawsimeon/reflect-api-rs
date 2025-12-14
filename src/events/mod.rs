use axum::Router;
use crate::AppState;

pub mod get_recent_events;
pub mod get_events_by_signer;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/recent", axum::routing::get(get_recent_events::get_recent_events))
        .route("/by-signer", axum::routing::get(get_events_by_signer::get_events_by_signer))
}
