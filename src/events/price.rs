use serde::Deserialize;

use crate::types::Object;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct PriceFundingUpdateDetails {
    pub funding_index: String,
    pub funding_timestamp_us: String,
    pub outstanding_funding_index: String,
    pub outstanding_funding_timestamp_us: String,
    pub funding_period_us: String,
    pub instant_daily_funding_rate: String,
}

/// Emitted whenever a price is updated.
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct PriceUpdateEvent {
    pub market: Object,
    /// Multiplied by the px_decimal of the collateral balance sheet
    pub oracle_px: String,
    /// Multiplied by the px_decimal of the collateral balance sheet
    pub mark_px: String,
    pub impact_ask_px: String,
    pub impact_bid_px: String,
    /// Deprecated in V2 events — use `funding` field instead
    pub funding_index: Option<String>,
    /// Deprecated in V2 events — use `funding` field instead
    pub funding_rate_bps: Option<String>,
    /// V2 funding details (present when funding model is updated)
    pub funding: Option<PriceFundingUpdateDetails>,
}
