use crate::event::Event;
use crate::events::*;

// Event type suffixes (after `{address}::`)
const ORDER_EVENT_TYPE: &str = "market_types::OrderEvent";
const BULK_ORDER_PLACED_EVENT_TYPE: &str = "market_types::BulkOrderPlacedEvent";
const BULK_ORDER_MODIFIED_EVENT_TYPE: &str = "market_types::BulkOrderModifiedEvent";
const BULK_ORDER_FILL_EVENT_TYPE: &str = "market_types::BulkOrderFilledEvent";
const BULK_ORDER_REJECTION_EVENT_TYPE: &str = "market_types::BulkOrderRejectionEvent";
const MARKET_REG_EVENT_TYPE: &str = "perp_engine::MarketRegistrationEvent";
const MARKET_UPDATE_EVENT_TYPE: &str = "perp_engine::MarketUpdateEvent";
const POSITION_UPDATE_EVENT_TYPE: &str = "perp_positions::PositionUpdateEvent";
const PRICE_UPDATE_EVENT_TYPE: &str = "price_management::PriceUpdateEvent";
const COLLATERAL_BALANCE_CHANGE_EVENT_TYPE: &str =
    "collateral_balance_sheet::CollateralBalanceChangeEvent";
const DEX_REG_EVENT_TYPE: &str = "perp_engine::DexRegistrationEvent";
const SUBACCOUNT_CREATED_EVENT_TYPE: &str = "dex_accounts::SubaccountCreatedEvent";
const DELEGATION_CHANGED_EVENT_TYPE: &str = "dex_accounts::DelegationChangedEvent";
const SUBACCOUNT_ACTIVE_CHANGED_EVENT_TYPE: &str = "dex_accounts::SubaccountActiveChangedEvent";
const TRADE_EVENT_TYPE: &str = "perp_positions::TradeEvent";
const TWAP_EVENT_TYPE: &str = "async_matching_engine::TwapEvent";
const OPEN_INTEREST_UPDATE_EVENT_TYPE: &str = "open_interest_tracker::OpenInterestUpdateEvent";
const MARKET_MODE_CHANGED_EVENT_TYPE: &str = "perp_market_config::MarketStatusChangeEvent";
const VAULT_CREATED_EVENT_TYPE: &str = "vault::VaultCreatedEvent";
const VAULT_ACTIVATED_EVENT_TYPE: &str = "vault::VaultActivatedEvent";
const VAULT_ADMIN_CHANGED_EVENT_TYPE: &str = "vault::VaultAdminChangedEvent";
const CONTRIBUTION_EVENT_TYPE: &str = "vault::ContributionEvent";
const REDEMPTION_INITIATED_EVENT_TYPE: &str = "vault::RedeemptionInitiatedEvent";
const REDEMPTION_SETTLED_EVENT_TYPE: &str = "vault::RedeemptionSettledEvent";
const FEE_DISTRIBUTION_EVENT_TYPE: &str = "vault::FeeDistributionEvent";
const GLOBAL_VAULT_CONFIG_UPDATED_EVENT_TYPE: &str =
    "vault_global_config::GlobalVaultConfigUpdatedEvent";
const USDC_MINT_EVENT_TYPE: &str = "usdc::UsdcMintEvent";
const LIQUIDATION_EVENT_TYPE: &str = "liquidation::LiquidationEvent";
const MARGIN_CALL_LOG_EVENT_TYPE: &str = "liquidation::MarginCallLog";
const ACK_EVENT_TYPE: &str = "async_matching_engine::ACKEvent";
const REFERRAL_CODE_REGISTERED_EVENT_TYPE: &str =
    "referral_registry::ReferralCodeRegisteredEvent";
const REFERRAL_REGISTERED_EVENT_TYPE: &str = "referral_registry::ReferralRegisteredEvent";
const AFFILIATE_REGISTERED_EVENT_TYPE: &str = "referral_registry::AffiliateRegisteredEvent";

/// All recognized Decibel event types.
#[derive(Debug)]
pub enum KnownEvent {
    // --- Order events (use orderbook_address) ---
    OrderEvent(OrderEvent),
    BulkOrderUpdatedEvent(BulkOrderUpdatedEvent),
    BulkOrderFillEvent(BulkOrderFillEvent),
    BulkOrderRejectedEvent(BulkOrderRejectedEvent),
    // --- Trading events (use package_address) ---
    MarketUpdateEvent(MarketUpdateEvent),
    PositionUpdateEvent(PositionUpdateEvent),
    PriceUpdateEvent(PriceUpdateEvent),
    CollateralBalanceChangeEvent(CollateralBalanceChangeEvent),
    DexRegistrationEvent(DexRegistrationEvent),
    SubaccountCreatedEvent(SubaccountCreatedEvent),
    SubaccountActiveChangedEvent(SubaccountCreatedEvent),
    DelegationChangedEvent(DelegationChangedEvent),
    TradeEvent(TradeEvent),
    TwapEvent(TwapEvent),
    OpenInterestUpdateEvent(OpenInterestUpdateEvent),
    // --- Vault events ---
    VaultCreatedEvent(VaultCreatedEvent),
    VaultActivatedEvent(VaultActivatedEvent),
    VaultAdminChangedEvent(VaultAdminChangedEvent),
    ContributionEvent(ContributionEvent),
    RedeemptionInitiatedEvent(RedeemptionInitiatedEvent),
    RedeemptionSettledEvent(RedeemptionSettledEvent),
    FeeDistributionEvent(FeeDistributionEvent),
    GlobalVaultConfigUpdatedEvent(GlobalVaultConfigUpdatedEvent),
    // --- Market mode ---
    MarketModeChangedEvent(MarketModeChangedEvent),
    // --- Liquidation ---
    LiquidationEvent(LiquidationEvent),
    MarginCallLogEvent(MarginCallLogEvent),
    AckEvent(AckEvent),
    // --- USDC ---
    UsdcMintEvent(UsdcMintEvent),
    // --- Referral ---
    ReferralCodeRegisteredEvent(ReferralCodeRegisteredEvent),
    ReferralRegisteredEvent(ReferralRegisteredEvent),
    AffiliateRegisteredEvent(AffiliateRegisteredEvent),
    // --- Unknown ---
    Unknown(()),
}

/// Pre-computed event type strings to avoid per-event allocations.
pub struct EventTypeKeys {
    // Orderbook events
    pub order: String,
    pub bulk_order_placed: String,
    pub bulk_order_modified: String,
    pub bulk_order_fill: String,
    pub bulk_order_rejection: String,
    // Package events
    pub market_reg: String,
    pub market_update: String,
    pub position_update: String,
    pub price_update: String,
    pub collateral_balance_change: String,
    pub dex_reg: String,
    pub subaccount_created: String,
    pub subaccount_active_changed: String,
    pub delegation_changed: String,
    pub trade: String,
    pub twap: String,
    pub open_interest_update: String,
    pub market_mode_changed: String,
    pub vault_created: String,
    pub vault_activated: String,
    pub vault_admin_changed: String,
    pub contribution: String,
    pub redemption_initiated: String,
    pub redemption_settled: String,
    pub fee_distribution: String,
    pub global_vault_config_updated: String,
    pub liquidation: String,
    pub margin_call_log: String,
    pub ack: String,
    pub usdc_mint: String,
    pub referral_code_registered: String,
    pub referral_registered: String,
    pub affiliate_registered: String,
}

impl EventTypeKeys {
    pub fn new(package_addr: &str, orderbook_addr: &str) -> Self {
        let pkg = |suffix: &str| format!("{package_addr}::{suffix}");
        let ob = |suffix: &str| format!("{orderbook_addr}::{suffix}");
        Self {
            order: ob(ORDER_EVENT_TYPE),
            bulk_order_placed: ob(BULK_ORDER_PLACED_EVENT_TYPE),
            bulk_order_modified: ob(BULK_ORDER_MODIFIED_EVENT_TYPE),
            bulk_order_fill: ob(BULK_ORDER_FILL_EVENT_TYPE),
            bulk_order_rejection: ob(BULK_ORDER_REJECTION_EVENT_TYPE),
            market_reg: pkg(MARKET_REG_EVENT_TYPE),
            market_update: pkg(MARKET_UPDATE_EVENT_TYPE),
            position_update: pkg(POSITION_UPDATE_EVENT_TYPE),
            price_update: pkg(PRICE_UPDATE_EVENT_TYPE),
            collateral_balance_change: pkg(COLLATERAL_BALANCE_CHANGE_EVENT_TYPE),
            dex_reg: pkg(DEX_REG_EVENT_TYPE),
            subaccount_created: pkg(SUBACCOUNT_CREATED_EVENT_TYPE),
            subaccount_active_changed: pkg(SUBACCOUNT_ACTIVE_CHANGED_EVENT_TYPE),
            delegation_changed: pkg(DELEGATION_CHANGED_EVENT_TYPE),
            trade: pkg(TRADE_EVENT_TYPE),
            twap: pkg(TWAP_EVENT_TYPE),
            open_interest_update: pkg(OPEN_INTEREST_UPDATE_EVENT_TYPE),
            market_mode_changed: pkg(MARKET_MODE_CHANGED_EVENT_TYPE),
            vault_created: pkg(VAULT_CREATED_EVENT_TYPE),
            vault_activated: pkg(VAULT_ACTIVATED_EVENT_TYPE),
            vault_admin_changed: pkg(VAULT_ADMIN_CHANGED_EVENT_TYPE),
            contribution: pkg(CONTRIBUTION_EVENT_TYPE),
            redemption_initiated: pkg(REDEMPTION_INITIATED_EVENT_TYPE),
            redemption_settled: pkg(REDEMPTION_SETTLED_EVENT_TYPE),
            fee_distribution: pkg(FEE_DISTRIBUTION_EVENT_TYPE),
            global_vault_config_updated: pkg(GLOBAL_VAULT_CONFIG_UPDATED_EVENT_TYPE),
            liquidation: pkg(LIQUIDATION_EVENT_TYPE),
            margin_call_log: pkg(MARGIN_CALL_LOG_EVENT_TYPE),
            ack: pkg(ACK_EVENT_TYPE),
            usdc_mint: pkg(USDC_MINT_EVENT_TYPE),
            referral_code_registered: pkg(REFERRAL_CODE_REGISTERED_EVENT_TYPE),
            referral_registered: pkg(REFERRAL_REGISTERED_EVENT_TYPE),
            affiliate_registered: pkg(AFFILIATE_REGISTERED_EVENT_TYPE),
        }
    }
}

impl KnownEvent {
    /// Classify a raw Event into a KnownEvent by matching the event type string.
    /// Use `EventTypeKeys::new()` once at startup, then pass `keys` to each call.
    pub fn from_event(event: &Event, keys: &EventTypeKeys) -> Self {
        let t = event.type_.as_str();

        macro_rules! try_parse {
            ($ty:ty) => {
                match serde_json::from_value::<$ty>(event.data.clone()) {
                    Ok(parsed) => parsed,
                    Err(e) => {
                        tracing::warn!(
                            "Failed to parse {} at version={}, event_index={}: {}",
                            stringify!($ty),
                            event.transaction_version,
                            event.event_index,
                            e
                        );
                        return Self::Unknown(());
                    }
                }
            };
        }

        match t {
            s if s == keys.order => Self::OrderEvent(try_parse!(OrderEvent)),
            s if s == keys.bulk_order_placed => {
                Self::BulkOrderUpdatedEvent(try_parse!(BulkOrderUpdatedEvent))
            }
            s if s == keys.bulk_order_modified => {
                Self::BulkOrderUpdatedEvent(try_parse!(BulkOrderUpdatedEvent))
            }
            s if s == keys.bulk_order_fill => {
                Self::BulkOrderFillEvent(try_parse!(BulkOrderFillEvent))
            }
            s if s == keys.bulk_order_rejection => {
                Self::BulkOrderRejectedEvent(try_parse!(BulkOrderRejectedEvent))
            }
            s if s == keys.market_reg => Self::MarketUpdateEvent(try_parse!(MarketUpdateEvent)),
            s if s == keys.market_update => Self::MarketUpdateEvent(try_parse!(MarketUpdateEvent)),
            s if s == keys.position_update => {
                Self::PositionUpdateEvent(try_parse!(PositionUpdateEvent))
            }
            s if s == keys.price_update => Self::PriceUpdateEvent(try_parse!(PriceUpdateEvent)),
            s if s == keys.collateral_balance_change => {
                Self::CollateralBalanceChangeEvent(try_parse!(CollateralBalanceChangeEvent))
            }
            s if s == keys.dex_reg => Self::DexRegistrationEvent(try_parse!(DexRegistrationEvent)),
            s if s == keys.subaccount_created => {
                Self::SubaccountCreatedEvent(try_parse!(SubaccountCreatedEvent))
            }
            s if s == keys.subaccount_active_changed => {
                Self::SubaccountActiveChangedEvent(try_parse!(SubaccountCreatedEvent))
            }
            s if s == keys.delegation_changed => {
                Self::DelegationChangedEvent(try_parse!(DelegationChangedEvent))
            }
            s if s == keys.trade => Self::TradeEvent(try_parse!(TradeEvent)),
            s if s == keys.twap => Self::TwapEvent(try_parse!(TwapEvent)),
            s if s == keys.open_interest_update => {
                Self::OpenInterestUpdateEvent(try_parse!(OpenInterestUpdateEvent))
            }
            s if s == keys.market_mode_changed => {
                Self::MarketModeChangedEvent(try_parse!(MarketModeChangedEvent))
            }
            s if s == keys.vault_created => {
                Self::VaultCreatedEvent(try_parse!(VaultCreatedEvent))
            }
            s if s == keys.vault_activated => {
                Self::VaultActivatedEvent(try_parse!(VaultActivatedEvent))
            }
            s if s == keys.vault_admin_changed => {
                Self::VaultAdminChangedEvent(try_parse!(VaultAdminChangedEvent))
            }
            s if s == keys.contribution => {
                Self::ContributionEvent(try_parse!(ContributionEvent))
            }
            s if s == keys.redemption_initiated => {
                Self::RedeemptionInitiatedEvent(try_parse!(RedeemptionInitiatedEvent))
            }
            s if s == keys.redemption_settled => {
                Self::RedeemptionSettledEvent(try_parse!(RedeemptionSettledEvent))
            }
            s if s == keys.fee_distribution => {
                Self::FeeDistributionEvent(try_parse!(FeeDistributionEvent))
            }
            s if s == keys.global_vault_config_updated => {
                Self::GlobalVaultConfigUpdatedEvent(try_parse!(GlobalVaultConfigUpdatedEvent))
            }
            s if s == keys.liquidation => {
                Self::LiquidationEvent(try_parse!(LiquidationEvent))
            }
            s if s == keys.margin_call_log => {
                Self::MarginCallLogEvent(try_parse!(MarginCallLogEvent))
            }
            s if s == keys.ack => Self::AckEvent(try_parse!(AckEvent)),
            s if s == keys.usdc_mint => Self::UsdcMintEvent(try_parse!(UsdcMintEvent)),
            s if s == keys.referral_code_registered => {
                Self::ReferralCodeRegisteredEvent(try_parse!(ReferralCodeRegisteredEvent))
            }
            s if s == keys.referral_registered => {
                Self::ReferralRegisteredEvent(try_parse!(ReferralRegisteredEvent))
            }
            s if s == keys.affiliate_registered => {
                Self::AffiliateRegisteredEvent(try_parse!(AffiliateRegisteredEvent))
            }
            _ => Self::Unknown(()),
        }
    }
}
