use serde::{Deserialize, Serialize};

use crate::types::OptionType;

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "__variant__")]
#[allow(clippy::upper_case_acronyms)]
pub enum OrderStatusType {
    OPEN,
    FILLED,
    CANCELLED,
    REJECTED,
    #[allow(non_camel_case_types)]
    SIZE_REDUCED,
    ACKNOWLEDGED,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "__variant__")]
#[allow(clippy::upper_case_acronyms)]
pub enum TimeInForceVariant {
    GTC,
    #[allow(non_camel_case_types)]
    POST_ONLY,
    IOC,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "__variant__")]
pub enum CancellationReason {
    PostOnlyViolation,
    IOCViolation,
    PositionUpdateViolation,
    ReduceOnlyViolation,
    ClearinghouseSettleViolation,
    MaxFillLimitViolation,
    DuplicateClientOrderIdViolation,
    OrderPreCancelled,
    PlaceMakerOrderViolation,
    DeadMansSwitchExpired,
    DisallowedSelfTrading,
    OrderCancelledByUser,
    OrderCancelledBySystem,
    OrderCancelledBySystemDueToError,
    ClearinghouseStoppedMatching,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "__variant__", content = "_0")]
pub enum TriggerCondition {
    PriceMoveAbove(String),
    PriceMoveBelow(String),
    TimeBased(String),
}

/// Raw on-chain OrderEvent. Emitted whenever an order is created, updated, or cancelled.
/// Note: Orders use the `orderbook_address`, not `package_address`.
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct OrderEvent {
    pub parent: String,
    pub market: String,
    pub client_order_id: OptionType<String>,
    pub order_id: String,
    pub user: String,
    /// Multiplied by the sz_decimal of the market
    pub orig_size: String,
    /// Multiplied by the sz_decimal of the market
    pub remaining_size: String,
    /// Multiplied by the sz_decimal of the market
    pub size_delta: String,
    /// Multiplied by the px_decimal of the collateral balance sheet
    pub price: String,
    pub is_bid: bool,
    pub is_taker: bool,
    pub status: OrderStatusType,
    pub details: String,
    /// BCS-encoded metadata (contains reduce_only, builder_code, twap, tp/sl info)
    pub metadata_bytes: String,
    pub time_in_force: TimeInForceVariant,
    pub trigger_condition: OptionType<TriggerCondition>,
    pub cancellation_reason: OptionType<CancellationReason>,
}
