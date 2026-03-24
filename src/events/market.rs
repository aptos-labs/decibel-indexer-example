use serde::Deserialize;

use crate::types::Object;

/// Emitted whenever a market is registered or updated.
/// Both `MarketRegistrationEvent` and `MarketUpdateEvent` on-chain map to this struct.
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct MarketUpdateEvent {
    pub market: Object,
    pub name: String,
    pub sz_decimals: u8,
    pub max_leverage: u8,
    pub max_open_interest: String,
    /// Smallest allowable order size (normalized with sz_decimals)
    pub min_size: String,
    /// Minimum increment of order sizes (normalized with sz_decimals)
    pub lot_size: String,
    /// Minimum increment of order prices (normalized with collateral px_decimals)
    pub ticker_size: String,
    /// Haircut for unrealized PnL when calculating withdrawable balance (optional for backward compat)
    pub unrealized_pnl_haircut_bps: Option<String>,
}
