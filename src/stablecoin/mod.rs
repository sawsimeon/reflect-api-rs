use axum::Router;
use crate::AppState;

pub mod get_available_stablecoins;
pub mod get_supply_caps;
pub mod get_mint_redeem_quote;
pub mod generate_mint_transaction;
pub mod generate_burn_transaction;
pub mod get_all_apy;
pub mod get_latest_exchange_rates;
pub mod get_historical_exchange_rates;
pub mod get_specific_apy;
pub mod get_historical_apy;
pub mod get_realtime_exchange_rate;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", axum::routing::get(get_available_stablecoins::get_available_stablecoins))
        .route("/supply-caps", axum::routing::get(get_supply_caps::get_supply_caps))
        .route("/quote", axum::routing::post(get_mint_redeem_quote::get_mint_redeem_quote))
        .route("/mint/tx", axum::routing::post(generate_mint_transaction::generate_mint_transaction))
        .route("/burn/tx", axum::routing::post(generate_burn_transaction::generate_burn_transaction))
        .route("/apy", axum::routing::get(get_all_apy::get_all_apy))
        .route("/exchange-rates", axum::routing::get(get_latest_exchange_rates::get_latest_exchange_rates))
        .route("/:stablecoin/historical-rates", axum::routing::get(get_historical_exchange_rates::get_historical_exchange_rates))
        .route("/:stablecoin/apy", axum::routing::get(get_specific_apy::get_specific_apy))
        .route("/:stablecoin/historical-apy", axum::routing::get(get_historical_apy::get_historical_apy))
        .route("/:stablecoin/realtime-rate", axum::routing::get(get_realtime_exchange_rate::get_realtime_exchange_rate))
}
