use serde::Deserialize;

use crate::types::OptionType;

/// Emitted whenever a subaccount is created.
/// Also used for SubaccountActiveChangedEvent (same shape, different semantics).
#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct SubaccountCreatedEvent {
    pub subaccount: String,
    pub owner: String,
    pub is_primary: Option<bool>,
    pub is_active: Option<bool>,
    #[serde(default)]
    pub seed: OptionType<String>,
}

/// Permission types that can be delegated to other accounts
#[derive(Debug, Clone, serde::Serialize, Deserialize)]
#[serde(tag = "__variant__")]
pub enum PermissionType {
    TradePerpsAllMarkets,
    TradePerpsOnMarket { market: String },
    SubaccountFundsMovement,
    SubDelegate,
    TradeVaultTokens,
}

/// Emitted whenever a delegation is granted or revoked.
#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct DelegationChangedEvent {
    pub subaccount: String,
    pub delegated_account: String,
    /// If Some, the delegation was added or changed. If None, it was revoked.
    pub delegation: OptionType<PermissionType>,
    /// None when revoking a delegation
    pub expiration_time_s: OptionType<String>,
}
