use serde::{Deserialize, Serialize};

use crate::types::{Object, OptionType};

/// ActionVariant is emitted from TradeEvent.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "__variant__")]
pub enum ActionVariant {
    OpenLong,
    CloseLong,
    OpenShort,
    CloseShort,
    Net,
}

/// SourceVariant is emitted from TradeEvent.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "__variant__")]
#[allow(clippy::upper_case_acronyms)]
pub enum SourceVariant {
    OrderFill,
    MarginCall,
    BackStopLiquidation,
    ADL,
    MarketDelisted,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuilderCode {
    pub builder: String,
    pub fees: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderId {
    pub order_id: String,
}

fn default_source() -> SourceVariant {
    SourceVariant::OrderFill
}

fn default_is_taker() -> bool {
    false
}

/// Emitted whenever a trade is executed.
/// Note that during a fill event, two TradeEvents are emitted: one for the maker and one for the taker.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct TradeEvent {
    pub account: String,
    pub market: Object,
    pub action: ActionVariant,
    #[serde(default = "default_source")]
    pub source: SourceVariant,
    pub fill_id: String,
    #[serde(default = "default_is_taker")]
    pub is_taker: bool,
    /// Multiplied by the sz_decimal of the market
    pub size: String,
    /// Multiplied by the px_decimal of the collateral balance sheet
    pub price: String,
    /// Signed realized PnL (positive = profit, negative = loss)
    pub realized_pnl: String,
    /// Signed realized funding cost
    pub realized_funding_cost: String,
    /// Signed fee amount (negative = rebate)
    pub fee: String,
    pub order_id: OptionType<OrderId>,
    pub client_order_id: OptionType<String>,
    pub builder_code: OptionType<BuilderCode>,
}
