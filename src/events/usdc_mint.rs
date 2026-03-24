use serde::{Deserialize, Serialize};

/// Emitted when USDC is minted (testnet only).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct UsdcMintEvent {
    pub account: String,
    pub amount: String,
    pub is_restricted_mint: bool,
}
