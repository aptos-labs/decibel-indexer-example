use serde::{Deserialize, Serialize};

use crate::types::{Object, OptionType};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "__variant__")]
pub enum MarketMode {
    Open,
    ReduceOnly,
    AllowlistOnly,
    Halt,
    Delisting,
}

/// Emitted whenever a market's trading mode changes.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[allow(dead_code)]
pub struct MarketModeChangedEvent {
    pub market: Object,
    pub mode: MarketMode,
    pub reason: OptionType<String>,
}
