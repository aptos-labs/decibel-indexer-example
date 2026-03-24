use serde::{Deserialize, Serialize};

use crate::types::Object;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "__variant__")]
#[allow(clippy::upper_case_acronyms)]
pub enum LiquidationType {
    MarginCall,
    BackstopLiquidation,
    ADL,
}

/// DEPRECATED: Legacy event emitted by the older liquidation flow.
/// Retained for backward compatibility.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[allow(dead_code)]
pub struct LiquidationEvent {
    pub market: Object,
    #[serde(default)]
    pub is_isolated: bool,
    pub user: String,
    #[serde(rename = "type")]
    pub liquidation_type: LiquidationType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "__variant__")]
pub enum AckPhase {
    BackstopLiquidation,
    MarginCall,
    CheckADL,
    TriggerADL,
    CommitMarkPrice,
    QueueBackstopLiquidationsAndADL,
    QueueMarginCallLiquidations,
    InitialEnqueue,
}

/// Emitted when the async matching engine acknowledges a batch operation.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[allow(dead_code)]
pub struct AckEvent {
    pub market: Object,
    pub accounts: Vec<String>,
    pub batch_key: String,
    pub ack_phase: AckPhase,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "__variant__")]
pub enum MarginCallResult {
    Solvent,
    RequiresBackstopLiquidation,
    Reprocess,
    Continuation,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "__variant__")]
pub enum MarginCallResultDetail {
    None,
    PositionNotFound,
    FeeExceedsHealthRatio,
    LiquidationPriceZero,
    EffectiveSlippageExceedsMMRRatio,
}

/// Detailed log of a margin call evaluation.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[allow(dead_code)]
pub struct MarginCallLogEvent {
    pub market: Object,
    pub mark_price: String,
    pub account: String,
    pub account_equity: String,
    pub maintenance_margin_required: String,
    pub slippage_to_test: String,
    pub effective_slippage: String,
    pub liquidation_price: String,
    pub units_to_liquidate: String,
    pub is_long: bool,
    pub size: String,
    pub units_liquidated: String,
    pub result: MarginCallResult,
    pub margin_call_result_detail: MarginCallResultDetail,
}
