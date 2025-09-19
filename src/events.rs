use aptos_indexer_processor_sdk::utils::convert::standardize_address;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Object {
    pub inner: String,
}

impl Object {
    #[allow(dead_code)]
    pub fn object_addr(self) -> String {
        standardize_address(&self.inner)
    }
}

// Emitted upon the registration of a new dex.
#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct DexRegistrationEvent {
    pub dex: Object,
    pub collateral_asset: Object,
    pub collateral_balance_decimals: u8,
}

// Emitted whenever a subaccount is created.
#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct SubaccountCreatedEvent {
    pub subaccount: String,
    pub owner: String,
    pub is_primary: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "__variant__")]
pub enum OrderStatusType {
    OPEN,
    FILLED,
    CANCELLED,
    REJECTED,
    #[allow(non_camel_case_types)]
    SIZE_REDUCED,
    ACKNOWLEDGED,
}

// Emitted whenever an order is created, updated, or cancelled.
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct OrderEvent {
    pub parent: String,
    pub market: String,
    pub order_id: String,
    pub user: String,
    // multiplied by the sz_decimal of the market
    pub orig_size: String,
    // multiplied by the sz_decimal of the market
    pub remaining_size: String,
    // multiplied by the sz_decimal of the market
    pub size_delta: String,
    // multiplied by the px_decimal of the collateral balance sheet.
    pub price: String,
    pub is_bid: bool,
    pub is_taker: bool,
    pub status: OrderStatusType,
    pub details: String,
}

// ActionVariant from decibel trade.rs
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "__variant__")]
pub enum ActionVariant {
    OpenLong,
    CloseLong,
    OpenShort,
    CloseShort,
    Net,
}

// Emitted whenever a trade is executed. Note that during a fill event, two TradeEvents are emitted:
// one for the maker and one for the taker.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TradeEvent {
    pub account: String,
    pub market: Object,
    pub action: ActionVariant,
    // multiplied by the sz_decimal of the market
    pub size: String,
    // multiplied by the px_decimal of the collateral balance sheet.
    pub price: String,
    pub is_profit: bool,
    // multiplied by the px_decimal of the collateral balance sheet.
    pub realized_pnl_amount: String,
    pub is_funding_positive: bool,
    // multiplied by the px_decimal of the collateral balance sheet.
    pub realized_funding_amount: String,
    pub is_rebate: bool,
    // multiplied by the px_decimal of the collateral balance sheet.
    pub fee_amount: String,
}

// Emitted whenever a market is registered with the dex.
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct MarketRegistrationEvent {
    pub market: Object,
    pub name: String,
    pub sz_decimals: u8,
    pub px_decimals: u8,
    pub max_leverage: u8,
    pub max_open_interest: String,
}

// Emitted whenever a position is updated.
#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct PositionUpdateEvent {
    pub market: Object,
    pub user: String,
    pub is_long: bool,
    // multiplied by the sz_decimal of the market
    pub size: String,
    pub user_leverage: u8,
    pub max_allowed_leverage: u8,
    pub funding_index: Option<String>,
    pub unrealized_funding_amount: Option<String>,
    pub is_funding_positive: Option<bool>,
    // multiplied by the sz_decimal of the market and the px_decimal of the collateral balance sheet.
    // This would represent the notional value of the position.
    pub entry_price_times_size_sum: String,
    pub is_isolated: bool,
    pub updated_funding_index: Option<String>,
    pub unrealized_funding_cost: Option<String>,
    pub funding_index_at_last_update: Option<String>,
    // in units of collateral balance sheet.
    pub unrealized_funding_amount_before_last_update: Option<String>,
    pub is_unrealized_funding_amount_before_last_update_positive: bool,
}

// Emitted whenever a price is updated.
#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct PriceUpdateEvent {
    pub market: Object,
    // multiplied by the px_decimal of the collateral balance sheet.
    pub oracle_px: String,
    // multiplied by the px_decimal of the collateral balance sheet.
    pub mark_px: String,
    pub impact_ask_px: String,
    pub impact_bid_px: String,
    pub funding_index: String,
    pub funding_rate_bps: String,
    pub is_funding_positive: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "__variant__")]
#[allow(dead_code)]
pub enum CollateralBalanceType {
    Cross { account: String },
    Isolated { account: String, market: Object },
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "__variant__")]
pub enum CollateralBalanceChangeType {
    UserMovement,
    Fee,
    PnL,
    Margin,
    Liquidation,
}

// Emitted whenever an account's balance changes. An account can either be cross or isolated.
// When an account changes modes, one event will be emitted draining the entire balance,
// and another event will be emitted with the delta being the entire balance in the new mode.
#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct CollateralBalanceChangeEvent {
    pub balance_type: CollateralBalanceType,
    pub delta: String, // u64 as string
    pub is_delta_positive: bool,
    pub balance_after: AggregatorSnapshot,
    pub change_type: CollateralBalanceChangeType,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct AggregatorSnapshot {
    pub value: String, // u64 as string
}
