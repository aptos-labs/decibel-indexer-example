# Decibel Indexer Example

A barebones processor that parses Decibel events from the Aptos transaction stream.

## Customization

Add business logic to `process_transactions` in `events.rs` for quick prototyping. Refer to the Aptos Indexer SDK docs [here](https://aptos.dev/build/indexer/indexer-sdk) for more sophisticated indexing needs.

## Structure

- `src/main.rs` - Main entry point, processor setup, and transaction processing logic 
- `src/events.rs` - Event structures, parsing logic, and helper functions.
- `src/decibel_object_events.rs` - Decibel specific event definitions.

## Usage
1. **Clone and setup**:
   ```bash
   cd decibel-indexer-example
   cp example-config.yaml config.yaml
   # Edit config.yaml with your auth token
   ```

   **Note**: You'll need an auth token for connecting to gRPC stream. Contact the Aptos team for access tokens.

2. **Run the processor**:
   ```bash
   cargo run -- -c config.yaml
   ```

## Event Types

This processor recognizes all the following event types from Decibel:

- `market_types::OrderEvent` - Order-related events (from 0x7 package)
- `perp_positions::TradeEvent` - Trade execution events
- `perp_engine::MarketRegistrationEvent` - Market registration events
- `perp_positions::PositionUpdateEvent` - Position update events
- `price_management::PriceUpdateEvent` - Price update events
- `collateral_balance_sheet::CollateralBalanceChangeEvent` - Collateral balance changes
- `perp_engine::DexRegistrationEvent` - DEX registration events
- `dex_accounts::SubaccountCreatedEvent` - Subaccount creation events