use serde::Deserialize;

use crate::types::{Object, OptionType};

/// Fixed-sized take-profit/stop-loss
#[derive(Debug, Clone, serde::Serialize, Deserialize, PartialEq)]
pub struct FixedSizedTpSlForEvent {
    pub order_id: String,
    pub trigger_price: String,
    pub limit_price: OptionType<String>,
    pub size: String,
}

/// Full-sized take-profit/stop-loss
#[derive(Debug, Clone, serde::Serialize, Deserialize, PartialEq)]
pub struct FullSizedTpSlForEvent {
    pub order_id: String,
    pub trigger_price: String,
    pub limit_price: OptionType<String>,
}

/// Emitted whenever a position is updated.
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct PositionUpdateEvent {
    pub market: Object,
    pub user: String,
    pub is_long: bool,
    /// Multiplied by the sz_decimal of the market
    pub size: String,
    pub user_leverage: u8,
    /// Multiplied by the sz_decimal of the market and the px_decimal of the collateral balance sheet.
    /// Represents the notional value of the position.
    pub entry_price_times_size_sum: String,
    pub is_isolated: bool,
    pub funding_index_at_last_update: Option<String>,
    /// In units of collateral balance sheet
    pub unrealized_funding_amount_before_last_update: Option<String>,
    pub full_sized_tp: OptionType<FullSizedTpSlForEvent>,
    pub fixed_sized_tps: Vec<FixedSizedTpSlForEvent>,
    pub full_sized_sl: OptionType<FullSizedTpSlForEvent>,
    pub fixed_sized_sls: Vec<FixedSizedTpSlForEvent>,
}
