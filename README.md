# Decibel Indexer Example

A reference processor that parses all Decibel DEX events from the Aptos transaction stream. Use this as a starting point for building your own indexer, analytics pipeline, or trading bot.

## Prerequisites

- [Rust](https://rustup.rs/) 1.70+
- A free gRPC auth token from the [Aptos Developer Portal](https://developers.aptoslabs.com/)

## Quick Start

```bash
git clone https://github.com/aptos-labs/decibel-indexer-example.git
cd decibel-indexer-example
cp example-config.yaml config.yaml
# Edit config.yaml — set your auth_token
cargo run -- -c config.yaml
```

## Contract Addresses

| Network | Package Address | Orderbook Address |
|---------|----------------|-------------------|
| Testnet | `0xe7da2794b1d8af76532ed95f38bfdf1136abfd8ea3a240189971988a83101b7f` | same as package |
| Mainnet | `0x50ead22afd6ffd9769e3b3d6e0e64a2a350d68e8b102c4e72e33d0b8cfdfdb06` | same as package |

On both testnet and mainnet, the orderbook is published under the same address as the package.

## Configuration

| Field | Description |
|-------|-------------|
| `indexer_grpc_data_service_address` | Aptos gRPC endpoint (see network configs below) |
| `starting_version` | Transaction version to start indexing from |
| `ending_version` | Optional end version (`null` = run forever) |
| `package_address` | Decibel DEX contract address |
| `orderbook_address` | Orderbook contract address (same as package on testnet/mainnet) |
| `auth_token` | gRPC auth token from [Aptos Developer Portal](https://developers.aptoslabs.com/) |

### Testnet

```yaml
indexer_grpc_data_service_address: "https://grpc.testnet.aptoslabs.com:443"
starting_version: 8106081556
package_address: "0xe7da2794b1d8af76532ed95f38bfdf1136abfd8ea3a240189971988a83101b7f"
orderbook_address: "0xe7da2794b1d8af76532ed95f38bfdf1136abfd8ea3a240189971988a83101b7f"
```

### Mainnet

```yaml
indexer_grpc_data_service_address: "https://grpc.mainnet.aptoslabs.com:443"
starting_version: 4365621793
package_address: "0x50ead22afd6ffd9769e3b3d6e0e64a2a350d68e8b102c4e72e33d0b8cfdfdb06"
orderbook_address: "0x50ead22afd6ffd9769e3b3d6e0e64a2a350d68e8b102c4e72e33d0b8cfdfdb06"
```

The `starting_version` values above are the first transactions containing Decibel events on each network. You can set a higher version to skip already-processed history.

## Structure

```
src/
├── main.rs           # Processor setup, pipeline, and match arms for all events
├── types.rs          # Shared types: Object, OptionType<T>, timestamp helper
├── event.rs          # Event struct, extraction from transactions
├── event_router.rs   # KnownEvent enum (31 variants), classification logic
└── events/           # All event type definitions
    ├── mod.rs
    ├── order.rs          # OrderEvent + enums (status, time-in-force, trigger, cancellation)
    ├── bulk_order.rs     # BulkOrderUpdated/Fill/Rejected
    ├── trade.rs          # TradeEvent (action, source, fill_id, pnl, fee, builder_code)
    ├── position.rs       # PositionUpdateEvent + TP/SL types
    ├── price.rs          # PriceUpdateEvent + PriceFundingUpdateDetails
    ├── collateral.rs     # CollateralBalanceChangeEvent + I64Snapshot (offset balance)
    ├── market.rs         # MarketUpdateEvent (min_size, lot_size, ticker_size)
    ├── dex.rs            # DexRegistrationEvent
    ├── subaccount.rs     # SubaccountCreatedEvent + DelegationChangedEvent
    ├── twap.rs           # TwapEvent + status enums
    ├── open_interest.rs  # OpenInterestUpdateEvent
    ├── vault.rs          # 7 vault events + GlobalVaultConfig
    ├── liquidation.rs    # MarginCallLogEvent, AckEvent (+ deprecated LiquidationEvent)
    ├── referral.rs       # 3 referral events (V1 enum wrappers)
    ├── usdc_mint.rs      # UsdcMintEvent (testnet only)
    └── market_mode.rs    # MarketModeChangedEvent + MarketMode enum
```

## Event Catalog (31 types)

### Trading Core
| Event | Module | Address | Description |
|-------|--------|---------|-------------|
| `OrderEvent` | `market_types` | orderbook | Order lifecycle (create, fill, cancel, reject) |
| `BulkOrderPlacedEvent` | `market_types` | orderbook | Bulk order placed |
| `BulkOrderModifiedEvent` | `market_types` | orderbook | Bulk order modified |
| `BulkOrderFilledEvent` | `market_types` | orderbook | Bulk order fill |
| `BulkOrderRejectionEvent` | `market_types` | orderbook | Bulk order rejected |
| `TradeEvent` | `perp_positions` | package | Trade execution (includes source, pnl, fee, builder) |
| `PositionUpdateEvent` | `perp_positions` | package | Position state change (includes TP/SL arrays) |
| `TwapEvent` | `async_matching_engine` | package | TWAP order lifecycle |

### Market Data
| Event | Module | Address | Description |
|-------|--------|---------|-------------|
| `PriceUpdateEvent` | `price_management` | package | Oracle/mark price + funding (V1 & V2 compat) |
| `OpenInterestUpdateEvent` | `open_interest_tracker` | package | Market open interest changes |
| `MarketRegistrationEvent` | `perp_engine` | package | Market registered (same struct as MarketUpdate) |
| `MarketUpdateEvent` | `perp_engine` | package | Market parameters updated |
| `MarketStatusChangeEvent` | `perp_market_config` | package | Market mode (Open, ReduceOnly, Halt, etc.) |

### Account
| Event | Module | Address | Description |
|-------|--------|---------|-------------|
| `CollateralBalanceChangeEvent` | `collateral_balance_sheet` | package | Balance change (offset-balance model) |
| `DexRegistrationEvent` | `perp_engine` | package | DEX initialization |
| `SubaccountCreatedEvent` | `dex_accounts` | package | New subaccount |
| `SubaccountActiveChangedEvent` | `dex_accounts` | package | Subaccount activation/deactivation |
| `DelegationChangedEvent` | `dex_accounts` | package | Session key delegation grant/revoke |

### Vaults
| Event | Module | Address | Description |
|-------|--------|---------|-------------|
| `VaultCreatedEvent` | `vault` | package | New vault created |
| `VaultActivatedEvent` | `vault` | package | Vault activated for trading |
| `VaultAdminChangedEvent` | `vault` | package | Vault admin transferred |
| `ContributionEvent` | `vault` | package | User deposited into vault |
| `RedeemptionInitiatedEvent` | `vault` | package | Redemption requested |
| `RedeemptionSettledEvent` | `vault` | package | Redemption settled |
| `FeeDistributionEvent` | `vault` | package | Performance fee distributed |
| `GlobalVaultConfigUpdatedEvent` | `vault_global_config` | package | Global vault parameters changed |

### Liquidation
| Event | Module | Address | Description |
|-------|--------|---------|-------------|
| `LiquidationEvent` *(deprecated)* | `liquidation` | package | Legacy liquidation event |
| `MarginCallLog` | `liquidation` | package | Margin call evaluation details |
| `ACKEvent` | `async_matching_engine` | package | Async matching acknowledgement |

### Other
| Event | Module | Address | Description |
|-------|--------|---------|-------------|
| `UsdcMintEvent` | `usdc` | package | USDC faucet mint (testnet only) |
| `ReferralCodeRegisteredEvent` | `referral_registry` | package | Referral code created |
| `ReferralRegisteredEvent` | `referral_registry` | package | User registered with referral |
| `AffiliateRegisteredEvent` | `referral_registry` | package | Affiliate registered |

## Customization

Add your business logic in the `process_transactions` function in `src/main.rs`. Each `KnownEvent` match arm gives you a fully parsed, typed event struct. Common use cases:

- **Database indexer**: Write events to PostgreSQL/ClickHouse
- **Trading bot**: React to `TradeEvent`, `PriceUpdateEvent`, `OrderEvent`
- **Analytics**: Aggregate `CollateralBalanceChangeEvent`, `OpenInterestUpdateEvent`
- **Notifications**: Alert on `LiquidationEvent`, `MarginCallLogEvent`

For more sophisticated indexing needs, see the [Aptos Indexer SDK docs](https://aptos.dev/build/indexer/indexer-sdk).

## Move Serialization Notes

- **Enums**: Move enums serialize with `"__variant__": "VariantName"` tag
- **Option**: Move's `Option<T>` serializes as `{ "vec": [] }` (None) or `{ "vec": [value] }` (Some). Use the `OptionType<T>` wrapper to deserialize.
- **Numbers**: Large numbers (u64, u128) are serialized as strings in JSON
- **Addresses**: Raw hex strings — use `standardize_address()` to normalize to 0x-prefixed, 66-char format

## Troubleshooting

| Problem | Cause | Fix |
|---------|-------|-----|
| `Unauthenticated` / 401 | Missing or invalid auth token | Get a token from [developers.aptoslabs.com](https://developers.aptoslabs.com/) |
| Runs but no events logged | Wrong `starting_version` or `package_address` | Use the starting versions and addresses listed above |
| Parse warnings in logs | Event schema changed on-chain | Check for updates to this repo |
