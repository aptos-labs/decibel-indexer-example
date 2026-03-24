use serde::{Deserialize, Serialize};

use crate::types::Object;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct VaultCreatedEvent {
    pub vault: String,
    pub creator: String,
    pub vault_name: String,
    pub vault_description: String,
    pub vault_social_links: Vec<String>,
    pub vault_share_symbol: String,
    pub contribution_asset_type: Object,
    pub share_asset_type: Object,
    pub contribution_lockup_duration_s: String,
    pub dex_primary_subaccount: Option<String>,
    pub fee_bps: String,
    pub fee_recipient: Option<String>,
    pub fee_interval_s: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct VaultActivatedEvent {
    pub vault: Object,
    pub num_shares: String,
    pub nav: String,
    pub respects_manager_minimum_shares_requirement: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct VaultAdminChangedEvent {
    pub vault: Object,
    pub admin: String,
    pub new_admin: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct ContributionEvent {
    pub vault: Object,
    pub user: String,
    pub assets_contributed: String,
    pub shares_received: String,
    pub unlock_time_s: String,
    #[serde(default)]
    pub reduction_shares: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct RedeemptionInitiatedEvent {
    pub vault: Object,
    pub user: String,
    pub shares_to_redeem: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct RedeemptionSettledEvent {
    pub vault: Object,
    pub user: String,
    pub shares_redeemed: String,
    pub assets_received: String,
    pub outstanding_fee_shares_paid: String,
    pub slippage_adjustment_bps: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct FeeDistributionEvent {
    pub vault: Object,
    pub fee_recipient: String,
    pub previous_nav: String,
    pub previous_shares: String,
    pub current_nav: String,
    pub current_shares: String,
    pub current_fee_amount: String,
    pub accrued_during_period_fee_shares: String,
    pub shares_received: String,
    pub diluted_shares: String,
    pub min_shares_for_manager: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct GlobalVaultConfigUpdatedEvent {
    pub max_fee_bps: String,
    pub min_fee_interval: String,
    pub max_fee_interval: String,
    pub creation_fee: String,
    pub creation_fee_recipient: String,
    pub min_funds_for_activation: String,
    pub min_manager_funds_fraction_bps: Option<String>,
    pub min_manager_funds_amount: Option<String>,
}
