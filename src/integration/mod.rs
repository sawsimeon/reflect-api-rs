use axum::Router;
use crate::AppState;

pub mod initialize_integration;
pub mod initialize_stablecoin_token;
pub mod transfer_mint_authority;
pub mod initialize_integration_flow;
pub mod get_integration_config;
pub mod update_integration_config;
pub mod get_integrations_by_authority;
pub mod upload_integration_metadata;
pub mod reveal_api_key;
pub mod rotate_api_key;
pub mod whitelist_users;
pub mod get_integration_statistics;
pub mod get_integration_events;
pub mod get_historical_integration_stats;
pub mod get_current_exchange_rate;
pub mod initialize_integration_vault;
pub mod initialize_user_branded_token;
pub mod generate_integration_mint_tx;
pub mod mint_and_whitelabel;
pub mod generate_redemption_tx;
pub mod redeem_whitelabeled;
pub mod generate_claim_tx;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/init", axum::routing::post(initialize_integration::initialize_integration))
        .route("/token/init", axum::routing::post(initialize_stablecoin_token::initialize_stablecoin_token))
        .route("/transfer-authority", axum::routing::post(transfer_mint_authority::transfer_mint_authority))
        .route("/flow/init", axum::routing::post(initialize_integration_flow::initialize_flow))
        .route("/config", axum::routing::get(get_integration_config::get_integration_config))
        .route("/config/update", axum::routing::post(update_integration_config::update_integration_config))
        .route("/by-authority", axum::routing::get(get_integrations_by_authority::get_integrations_by_authority))
        .route("/metadata/upload", axum::routing::post(upload_integration_metadata::upload_integration_metadata))
        .route("/api-key/reveal", axum::routing::post(reveal_api_key::reveal_api_key))
        .route("/api-key/rotate", axum::routing::post(rotate_api_key::rotate_api_key))
        .route("/whitelist", axum::routing::post(whitelist_users::whitelist_users))
        .route("/stats", axum::routing::get(get_integration_statistics::get_integration_statistics))
        .route("/events", axum::routing::get(get_integration_events::get_integration_events))
        .route("/historical-stats", axum::routing::get(get_historical_integration_stats::get_historical_integration_stats))
        .route("/exchange-rate", axum::routing::get(get_current_exchange_rate::get_current_exchange_rate))
        .route("/vault/init", axum::routing::post(initialize_integration_vault::initialize_integration_vault))
        .route("/user-token/init", axum::routing::post(initialize_user_branded_token::initialize_user_branded_token))
        .route("/mint/tx", axum::routing::post(generate_integration_mint_tx::generate_integration_mint_tx))
        .route("/mint-whitelabel", axum::routing::post(mint_and_whitelabel::mint_and_whitelabel))
        .route("/redeem/tx", axum::routing::post(generate_redemption_tx::generate_redemption_tx))
        .route("/redeem-whitelabel", axum::routing::post(redeem_whitelabeled::redeem_whitelabeled))
        .route("/claim/tx", axum::routing::post(generate_claim_tx::generate_claim_tx))
}
