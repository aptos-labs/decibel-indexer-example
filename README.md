# Decibel Indexer Example

A simple barebones processor that parses events from the Aptos blockchain, specifically designed to demonstrate event processing patterns used in the Etna indexer.

## Features

- **Event Extraction**: Extracts and parses events from Aptos transactions
- **Event Processing**: Processes specific event types (OrderEvent, TradeEvent, MarketRegistrationEvent)  
- **Logging**: Provides detailed logging of processed events and metadata
- **No Database Dependencies**: Simple implementation that just parses and logs events

## Structure

- `src/main.rs` - Main entry point, processor setup, and transaction processing logic (~227 lines)
- `src/events.rs` - Event structures, parsing logic, and helper functions (~361 lines)
- `example-config.yaml` - Example configuration file
- `config.yaml` - Working configuration

## Configuration

Copy `example-config.yaml` to `config.yaml` and update the settings:

```yaml
# Aptos GRPC endpoint
indexer_grpc_data_service_address: "https://grpc.netna.aptoslabs.com:443"

# Starting version (optional)
starting_version: 0

# Ending version (optional, null = run forever)
ending_version: null

package_address: "0xb8a5788314451ce4d2fbbad32e1bad88d4184b73943b7fe5166eab93cf1a5a95"
auth_token: "your-auth-token-here"
```

## Usage

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone and setup**:
   ```bash
   cd decibel-indexer-example
   cp example-config.yaml config.yaml
   # Edit config.yaml with your package address and auth token
   ```

   **Note**: For staging endpoints like `grpc.netna.staging.aptoslabs.com`, you'll need an auth token. Contact the Aptos team or check your developer console for access tokens.

3. **Run the processor**:
   ```bash
   cargo run -- -c config.yaml
   ```

## Event Types

This processor recognizes all the following event types from Etna:

- `market_types::OrderEvent` - Order-related events (from 0x7 package)
- `perp_positions::TradeEvent` - Trade execution events
- `perp_engine::MarketRegistrationEvent` - Market registration events
- `perp_positions::PositionUpdateEvent` - Position update events
- `price_management::PriceUpdateEvent` - Price update events
- `collateral_balance_sheet::CollateralBalanceChangeEvent` - Collateral balance changes
- `perp_engine::DexRegistrationEvent` - DEX registration events
- `dex_accounts::SubaccountCreatedEvent` - Subaccount creation events

Events not matching these patterns are logged as "Unknown" events.

## Customization

To adapt this for your own package:

1. **Update event type constants** in `src/events.rs`:
   ```rust
   pub const ORDER_EVENT_TYPE: &str = "your_module::YourOrderEvent";
   ```

2. **Add custom processing logic** in the `process_transactions` function in `src/main.rs`