use serde::{Deserialize, Serialize};

use crate::types::OptionType;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ReferrerStateInner {
    pub codes: Vec<String>,
    pub max_referral_codes: OptionType<String>,
    pub max_usage_per_referral_code: OptionType<String>,
    pub is_affiliate: bool,
}

/// Move's ReferrerState enum
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "__variant__")]
pub enum ReferrerState {
    V1(ReferrerStateInner),
}

#[allow(dead_code)]
impl ReferrerState {
    pub fn inner(&self) -> &ReferrerStateInner {
        let Self::V1(inner) = self;
        inner
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ReferralCodeStateInner {
    pub code: String,
    pub num_referred: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "__variant__")]
pub enum ReferralCodeState {
    V1(ReferralCodeStateInner),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ReferralCodeRegisteredEventInner {
    pub referrer_addr: String,
    pub referral_code: String,
    pub referrer_state: ReferrerState,
}

impl Default for ReferrerState {
    fn default() -> Self {
        Self::V1(ReferrerStateInner::default())
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ReferralRegisteredEventInner {
    pub referral_code: String,
    pub referree_addr: String,
    pub referrer_addr: String,
    pub referral_code_state: ReferralCodeState,
}

impl Default for ReferralCodeState {
    fn default() -> Self {
        Self::V1(ReferralCodeStateInner::default())
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AffiliateRegisteredEventInner {
    pub affiliate_addr: String,
    pub referrer_state: ReferrerState,
}

/// Emitted when a referral code is registered.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "__variant__")]
pub enum ReferralCodeRegisteredEvent {
    V1(ReferralCodeRegisteredEventInner),
}

/// Emitted when a user registers with a referral code.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "__variant__")]
pub enum ReferralRegisteredEvent {
    V1(ReferralRegisteredEventInner),
}

/// Emitted when an affiliate is registered.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "__variant__")]
pub enum AffiliateRegisteredEvent {
    V1(AffiliateRegisteredEventInner),
}
