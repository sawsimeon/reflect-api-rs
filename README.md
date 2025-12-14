# Reflect API (reflect-api-rs) <img src="https://pbs.twimg.com/profile_images/1915772772770291712/WXX_G2-J_400x400.jpg" alt="Reflect Logo" width="250" height="250">


**Unofficial Rust Library for the Reflect API**

This repository provides a modular, production-ready scaffold for building a backend server compatible with the **Reflect API** — the programmable infrastructure behind [Reflect](https://docs.reflect.money/reflect-api/), autonomous money designed for the stablecoin era.

Reflect issues income-generating stablecoins (such as **rUSD**, with future support for rEUR and others) that automatically farm the best DeFi rates on Solana while remaining fully liquid, fully insured on-chain, and non-custodial. Every stablecoin earns continuous yield with no lockups, no claims process, and built-in real-time insurance.

The official Reflect API (base URL: `[https://prod.api.reflect.money](https://docs.reflect.money/reflect-api/)`) enables developers and protocols to:
- Mint and redeem stablecoins
- Retrieve real-time/historical APYs and exchange rates
- Set up whitelabeled integrations with branded tokens
- Manage API keys, vaults, metadata, and user whitelists
- Generate on-chain transactions securely
- Access protocol statistics and events

This Rust implementation uses **Axum** as the web framework and is structured for clarity, maintainability, and future extension (e.g., adding database integration, authentication middleware, or on-chain interactions).

All endpoints are currently stubbed with placeholder/mock JSON responses, making this an ideal starting point for:
- Local development and testing
- Building a mirror/proxy server
- Creating a custom backend for Reflect integrations
- Learning the Reflect API structure

## Project scaffold

------------


    ├── Cargo.toml
    ├── src/
    │   ├── health/
    │   │   ├── health_check.rs
    │   │   └── mod.rs
    │   ├── stablecoin/
    │   │   ├── generate_burn_transaction.rs
    │   │   ├── generate_mint_transaction.rs
    │   │   ├── get_all_apy.rs
    │   │   ├── get_available_stablecoins.rs
    │   │   ├── get_historical_apy.rs
    │   │   ├── get_historical_exchange_rates.rs
    │   │   ├── get_latest_exchange_rates.rs
    │   │   ├── get_mint_redeem_quote.rs
    │   │   ├── get_realtime_exchange_rate.rs
    │   │   ├── get_specific_apy.rs
    │   │   ├── get_supply_caps.rs
    │   │   └── mod.rs
    │   ├── integration/
    │   │   ├── generate_claim_tx.rs
    │   │   ├── generate_integration_mint_tx.rs
    │   │   ├── generate_redemption_tx.rs
    │   │   ├── get_current_exchange_rate.rs
    │   │   ├── get_historical_integration_stats.rs
    │   │   ├── get_integration_config.rs
    │   │   ├── get_integration_events.rs
    │   │   ├── get_integration_statistics.rs
    │   │   ├── get_integrations_by_authority.rs
    │   │   ├── initialize_integration_flow.rs
    │   │   ├── initialize_integration_vault.rs
    │   │   ├── initialize_integration.rs
    │   │   ├── initialize_stablecoin_token.rs
    │   │   ├── initialize_user_branded_token.rs
    │   │   ├── mint_and_whitelabel.rs
    │   │   ├── mod.rs
    │   │   ├── redeem_whitelabeled.rs
    │   │   ├── reveal_api_key.rs
    │   │   ├── rotate_api_key.rs
    │   │   ├── transfer_mint_authority.rs
    │   │   ├── update_integration_config.rs
    │   │   ├── upload_integration_metadata.rs
    │   │   └── whitelist_users.rs
    │   ├── stats/
    │   │   ├── get_historical_tvl_and_volume.rs
    │   │   ├── get_protocol_statistics.rs
    │   │   └── mod.rs
    │   └── events/
    │       ├── get_events_by_signer.rs
    │       ├── get_recent_events.rs
    │       └── mod.rs
    ├── README.md

------------

Each endpoint lives in its own file for easy navigation and modification.

## Quick start

1. Clone and build:
```bash
   git clone https://github.com/sawsimeon/reflect-api-rs.git
   df reflect-api-rs
   cargo build
```
2. Run the server:
```bash
   cargo run
``` 

3. The server listens on 0.0.0.0:3000. Try some endpoints with curl:
```bash
- Root health
  - GET http://localhost:3000/ → { "status": "reflect api running" }
- Health
  - GET http://localhost:3000/health/ → { "status": "ok" }
- Stablecoins
  - GET http://localhost:3000/stablecoins/ → list of available stablecoins
  - GET http://localhost:3000/stablecoins/supply-caps → supply caps
  - POST http://localhost:3000/stablecoins/quote → body { "stablecoin": "rUSD", "amount": 10.0, "side": "mint" }
``` 

Endpoints are mounted under:
- /health
- /stablecoins
- /integrations
- /stats
- /events

Official Resources

Website: [https://www.reflect.money](https://www.reflect.money)
Official API Documentation: [https://docs.reflect.money/reflect-api](https://docs.reflect.money/reflect-api)
Full Documentaiton Hub: [https://docs.reflect.money](https://docs.reflect.money)

Note: This mirrors the official API structure but is a Rust implementation. However, it is unofficial and not affiliated with Reflect. Use the offical API ([https://prod.api.reflect.money](https://docs.reflect.money/reflect-api/)) in production. 



## Contributing

Contributions are welcome! Please open issues or pull requests for any improvements or bug fixes.

## License

MIT License: <https://opensource.org/license/MIT>


## Contact

For questions or feedback, please contact [Saw Simeon](mailto:sawsimeon@hotmail.com).
