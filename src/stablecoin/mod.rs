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
        // Stablecoin metadata
        .route(
            "/",
            axum::routing::get(get_available_stablecoins::get_available_stablecoins),
        )
        .route(
            "/supply-caps",
            axum::routing::get(get_supply_caps::get_supply_caps),
        )

        // Quotes
        .route(
            "/quote",
            axum::routing::post(get_mint_redeem_quote::get_mint_redeem_quote),
        )

        // Mint / Burn transactions
        .route(
            "/mint/tx",
            axum::routing::post(generate_mint_transaction::generate_mint_transaction),
        )
        .route(
            "/burn/tx",
            axum::routing::post(generate_burn_transaction::generate_burn_transaction),
        )

        // APY (all stablecoins)
        .route(
            "/apy",
            axum::routing::get(get_all_apy::get_all_apy),
        )

        // Latest exchange rates (all stablecoins)
        .route(
            "/exchange-rates",
            axum::routing::get(get_latest_exchange_rates::get_latest_exchange_rates),
        )

        // Historical exchange rates for a specific stablecoin
        .route(
            "/stablecoin/:index/exchange-rates/historical",
            axum::routing::get(get_historical_exchange_rates::get_historical_exchange_rates),
        )

        // Specific APY for a stablecoin
        .route(
            "/stablecoin/:index/apy",
            axum::routing::get(get_specific_apy::get_specific_apy),
        )

        // Historical APY for a stablecoin
        .route(
            "/stablecoin/:index/apy/historical",
            axum::routing::get(get_historical_apy::get_historical_apy),
        )

        // Realtime exchange rate for a stablecoin
        .route(
            "/stablecoin/:index/exchange-rate",
            axum::routing::get(get_realtime_exchange_rate::get_realtime_exchange_rate),
        )
}
