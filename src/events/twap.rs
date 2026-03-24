use serde::Deserialize;

use crate::types::{Object, OptionType};

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct TwapOrderIdType {
    pub order_id: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "__variant__")]
pub enum TwapOrderStatusType {
    Open,
    Triggered { _0: TwapOrderIdType, _1: String },
    Cancelled { _0: String },
}

/// Emitted whenever a TWAP order is created, triggered, or cancelled.
#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct TwapEvent {
    pub market: Object,
    pub account: String,
    pub is_buy: bool,
    pub order_id: TwapOrderIdType,
    pub is_reduce_only: bool,
    pub start_time_s: String,
    pub frequency_s: String,
    pub duration_s: String,
    pub orig_size: String,
    pub remaining_size: String,
    pub status: TwapOrderStatusType,
    pub client_order_id: OptionType<String>,
}
