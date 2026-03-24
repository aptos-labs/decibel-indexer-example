use serde::Deserialize;

use crate::types::Object;

/// Emitted whenever the open interest for a market changes.
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct OpenInterestUpdateEvent {
    pub market: Object,
    /// The current open interest for the market (long side only)
    pub current_open_interest: String,
}
