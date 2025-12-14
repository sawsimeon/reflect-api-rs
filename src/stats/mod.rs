use axum::Router;
use crate::AppState;

pub mod get_protocol_statistics;
pub mod get_historical_tvl_and_volume;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/protocol", axum::routing::get(get_protocol_statistics::get_protocol_statistics))
        .route("/historical", axum::routing::get(get_historical_tvl_and_volume::get_historical_tvl_and_volume))
}
