use serde::Deserialize;

use crate::types::OptionType;

use super::order::CancellationReason;

/// Emitted when a bulk order is placed or modified.
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct BulkOrderUpdatedEvent {
    pub market: String,
    pub account: String,
    pub sequence_number: String,
    pub ask_sizes: Vec<String>,
    pub ask_prices: Vec<String>,
    pub bid_sizes: Vec<String>,
    pub bid_prices: Vec<String>,
}

/// Emitted when a bulk order is filled.
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct BulkOrderFillEvent {
    pub market: String,
    pub account: String,
    pub sequence_number: String,
    pub is_bid: bool,
    pub price: String,
    pub fill_size: String,
    pub remaining_size: String,
}

/// Emitted when a bulk order is rejected.
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct BulkOrderRejectedEvent {
    pub market: String,
    pub account: String,
    pub sequence_number: String,
    pub cancellation_reason: OptionType<CancellationReason>,
}
