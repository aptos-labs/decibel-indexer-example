use serde::Deserialize;

use crate::types::Object;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct AggregatorSnapshot {
    pub value: String,
}

/// Offset-balance model for signed balances.
/// On-chain stores balance as u64 with offset of 2^63 (SIGNED_ZERO).
#[derive(Deserialize, Debug)]
#[serde(tag = "__variant__")]
#[allow(dead_code)]
pub enum I64Snapshot {
    V1 {
        offset_balance: AggregatorSnapshot,
    },
}

#[derive(Deserialize, Debug)]
#[serde(tag = "__variant__")]
#[allow(dead_code)]
pub enum CollateralBalanceType {
    Cross { account: String },
    Isolated { account: String, market: Object },
}

#[derive(Deserialize, Debug)]
#[serde(tag = "__variant__")]
pub enum CollateralBalanceChangeType {
    UserMovement,
    Fee,
    PnL,
    Margin,
    Liquidation,
}

/// Emitted whenever an account's balance changes. An account can either be cross or isolated.
/// When an account changes modes, one event will be emitted draining the entire balance,
/// and another event will be emitted with the delta being the entire balance in the new mode.
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct CollateralBalanceChangeEvent {
    pub asset_type: Object,
    pub balance_type: CollateralBalanceType,
    /// Signed delta as string
    pub delta: String,
    pub offset_balance_after: I64Snapshot,
    pub change_type: CollateralBalanceChangeType,
}
