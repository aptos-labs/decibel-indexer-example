use serde::Deserialize;

use crate::types::Object;

/// Emitted upon the registration of a new dex.
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct DexRegistrationEvent {
    pub dex: Object,
    pub collateral_asset: Object,
    pub collateral_balance_decimals: u8,
}
