use aptos_indexer_processor_sdk::{
    aptos_protos::transaction::v1::{transaction::TxnData, Event as EventPB, Transaction},
    utils::convert::{standardize_address, truncate_str},
};
use serde::{Deserialize, Serialize};

// Timestamp conversion helper (from etna models)
pub fn timestamp_to_unix_millis(
    timestamp: &aptos_indexer_processor_sdk::aptos_protos::util::timestamp::Timestamp,
) -> i64 {
    // Convert seconds to milliseconds and nanoseconds to milliseconds, then sum.
    // Timestamp nanos are the fractional nanoseconds from the last second, not UNIX nanoseconds.
    // We need to sum the two granularities together to get the full unix timestamp in milliseconds.
    (timestamp.seconds * 1000) + (timestamp.nanos / 1_000_000) as i64
}

// p99 currently is 303 so using 300 as a safe max length
const EVENT_TYPE_MAX_LENGTH: usize = 300;

// Event type constants from etna
const ORDER_EVENT_TYPE: &str = "market_types::OrderEvent";
const MARKET_REG_EVENT_TYPE: &str = "perp_engine::MarketRegistrationEvent";
const POSITION_UPDATE_EVENT_TYPE: &str = "perp_positions::PositionUpdateEvent";
const PRICE_UPDATE_EVENT_TYPE: &str = "price_management::PriceUpdateEvent";
const COLLATERAL_BALANCE_CHANGE_EVENT_TYPE: &str =
    "collateral_balance_sheet::CollateralBalanceChangeEvent";
const DEX_REG_EVENT_TYPE: &str = "perp_engine::DexRegistrationEvent";
const SUBACCOUNT_CREATED_EVENT_TYPE: &str = "dex_accounts::SubaccountCreatedEvent";
const TRADE_EVENT_TYPE: &str = "perp_positions::TradeEvent";

// Order event enums from etna
#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "__variant__")]
pub enum OrderStatusType {
    OPEN,
    FILLED,
    CANCELLED,
    REJECTED,
    #[allow(non_camel_case_types)]
    SIZE_REDUCED,
    ACKNOWLEDGED,
}
// Exact OrderEvent structure from etna
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct OrderEvent {
    pub parent: String,
    pub market: String,
    pub order_id: String,
    pub user: String,
    // multiplied by the sz_decimal of the market
    pub orig_size: String,
    // multiplied by the sz_decimal of the market
    pub remaining_size: String,
    // multiplied by the sz_decimal of the market
    pub size_delta: String,
    // multiplied by the px_decimal of the collateral balance sheet.
    pub price: String,
    pub is_bid: bool,
    pub is_taker: bool,
    pub status: OrderStatusType,
    pub details: String,
}

impl OrderEvent {
    pub fn from_event(event: &Event) -> Self {
        serde_json::from_value(event.data.clone()).unwrap()
    }
}

// ActionVariant from etna trade.rs
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "__variant__")]
pub enum ActionVariant {
    OpenLong,
    CloseLong,
    OpenShort,
    CloseShort,
    Net,
}

// Exact TradeEvent structure from etna
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TradeEvent {
    pub account: String,
    pub market: Object,
    pub action: ActionVariant,
    // multiplied by the sz_decimal of the market
    pub size: String,
    // multiplied by the px_decimal of the collateral balance sheet.
    pub price: String,
    pub is_profit: bool,
    // multiplied by the px_decimal of the collateral balance sheet.
    pub realized_pnl_amount: String,
    pub is_funding_positive: bool,
    // multiplied by the px_decimal of the collateral balance sheet.
    pub realized_funding_amount: String,
    pub is_rebate: bool,
    // multiplied by the px_decimal of the collateral balance sheet.
    pub fee_amount: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct MarketRegistrationEvent {
    pub market: Object,
    pub name: String,
    pub sz_decimals: u8,
    pub px_decimals: u8,
    pub max_leverage: u8,
    pub max_open_interest: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Object {
    pub inner: String,
}

impl Object {
    #[allow(dead_code)]
    pub fn object_addr(self) -> String {
        standardize_address(&self.inner)
    }
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct PositionUpdateEvent {
    pub market: Object,
    pub user: String,
    pub is_long: bool,
    // multiplied by the sz_decimal of the market
    pub size: String,
    pub user_leverage: u8,
    pub max_allowed_leverage: u8,
    pub funding_index: Option<String>,
    pub unrealized_funding_amount: Option<String>,
    pub is_funding_positive: Option<bool>,
    // multiplied by the sz_decimal of the market and the px_decimal of the collateral balance sheet.
    // This would represent the notional value of the position.
    pub entry_price_times_size_sum: String,
    pub is_isolated: bool,
    pub updated_funding_index: Option<String>,
    pub unrealized_funding_cost: Option<String>,
    pub funding_index_at_last_update: Option<String>,
    // in units of collateral balance sheet.
    pub unrealized_funding_amount_before_last_update: Option<String>,
    pub is_unrealized_funding_amount_before_last_update_positive: bool,
}
#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct PriceUpdateEvent {
    pub market: Object,
    // multiplied by the px_decimal of the collateral balance sheet.
    pub oracle_px: String,
    // multiplied by the px_decimal of the collateral balance sheet.
    pub mark_px: String,
    pub impact_ask_px: String,
    pub impact_bid_px: String,
    pub funding_index: String,
    pub funding_rate_bps: String,
    pub is_funding_positive: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct CollateralBalanceChangeEvent {
    pub balance_type: CollateralBalanceType,
    pub delta: String, // u64 as string
    pub is_delta_positive: bool,
    pub balance_after: AggregatorSnapshot,
    pub change_type: CollateralBalanceChangeType,
}
#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct AggregatorSnapshot {
    pub value: String, // u64 as string
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "__variant__")]
#[allow(dead_code)]
pub enum CollateralBalanceType {
    Cross { account: String },
    Isolated { account: String, market: Object },
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "__variant__")]
pub enum CollateralBalanceChangeType {
    UserMovement,
    Fee,
    PnL,
    Margin,
    Liquidation,
}
#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct DexRegistrationEvent {
    pub dex: Object,
    pub collateral_asset: Object,
    pub collateral_balance_decimals: u8,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct SubaccountCreatedEvent {
    pub subaccount: String,
    pub owner: String,
    pub is_primary: bool,
}

// Main Event struct similar to etna's Event
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Event {
    pub sequence_number: i64,
    pub creation_number: i64,
    pub account_address: String,
    pub transaction_version: i64,
    pub transaction_block_height: i64,
    pub transaction_unix_ms: i64,
    pub type_: String,
    pub data: serde_json::Value,
    pub event_index: i64,
    pub indexed_type: String,
}

impl Event {
    pub fn from_event(
        event: &EventPB,
        transaction_version: i64,
        transaction_block_height: i64,
        transaction_unix_ms: i64,
        event_index: i64,
    ) -> Self {
        let event_type: &str = event.type_str.as_ref();
        Event {
            account_address: standardize_address(
                event.key.as_ref().unwrap().account_address.as_str(),
            ),
            creation_number: event.key.as_ref().unwrap().creation_number as i64,
            sequence_number: event.sequence_number as i64,
            transaction_version,
            transaction_block_height,
            transaction_unix_ms,
            type_: event_type.to_string(),
            data: serde_json::from_str(event.data.as_str()).unwrap_or(serde_json::Value::Null),
            event_index,
            indexed_type: truncate_str(event_type, EVENT_TYPE_MAX_LENGTH),
        }
    }

    pub fn from_events(
        events: &[EventPB],
        transaction_version: i64,
        transaction_block_height: i64,
        transaction_unix_ms: i64,
    ) -> Vec<Self> {
        events
            .iter()
            .enumerate()
            .map(|(index, event)| {
                Self::from_event(
                    event,
                    transaction_version,
                    transaction_block_height,
                    transaction_unix_ms,
                    index as i64,
                )
            })
            .collect()
    }

    pub fn from_transaction(transaction: &Transaction) -> Vec<Self> {
        let txn_version = transaction.version as i64;
        let block_height = transaction.block_height as i64;
        let unix_ms = timestamp_to_unix_millis(transaction.timestamp.as_ref().unwrap());

        let txn_data = match &transaction.txn_data {
            Some(data) => data,
            None => return vec![],
        };
        let default = vec![];
        let raw_events = match txn_data {
            TxnData::Genesis(tx_inner) => &tx_inner.events,
            TxnData::User(tx_inner) => &tx_inner.events,
            _ => &default,
        };

        Event::from_events(raw_events, txn_version, block_height, unix_ms)
    }
}

// KnownEvent enum with all supported event types
#[derive(Debug)]
pub enum KnownEvent {
    OrderEvent(OrderEvent),
    MarketRegistrationEvent(MarketRegistrationEvent),
    PositionUpdateEvent(PositionUpdateEvent),
    PriceUpdateEvent(PriceUpdateEvent),
    CollateralBalanceChangeEvent(CollateralBalanceChangeEvent),
    DexRegistrationEvent(DexRegistrationEvent),
    SubaccountCreatedEvent(SubaccountCreatedEvent),
    TradeEvent(TradeEvent),
    Unknown(()),
}
impl KnownEvent {
    pub fn from_event(event: &Event, package_addr: &str) -> Self {
        let event_type = event.type_.clone();
        match event_type.as_str() {
            s if s == format!("0x7::{}", ORDER_EVENT_TYPE) => {
                Self::OrderEvent(OrderEvent::from_event(event))
            }
            s if s == format!("{}::{}", package_addr, MARKET_REG_EVENT_TYPE) => {
                let parsed: MarketRegistrationEvent =
                    serde_json::from_value(event.data.clone()).unwrap();
                Self::MarketRegistrationEvent(parsed)
            }
            s if s == format!("{}::{}", package_addr, POSITION_UPDATE_EVENT_TYPE) => {
                let parsed: PositionUpdateEvent =
                    serde_json::from_value(event.data.clone()).unwrap();
                Self::PositionUpdateEvent(parsed)
            }
            s if s == format!("{}::{}", package_addr, PRICE_UPDATE_EVENT_TYPE) => {
                let parsed: PriceUpdateEvent = serde_json::from_value(event.data.clone()).unwrap();
                Self::PriceUpdateEvent(parsed)
            }
            s if s == format!("{}::{}", package_addr, COLLATERAL_BALANCE_CHANGE_EVENT_TYPE) => {
                let parsed: CollateralBalanceChangeEvent =
                    serde_json::from_value(event.data.clone()).unwrap();
                Self::CollateralBalanceChangeEvent(parsed)
            }
            s if s == format!("{}::{}", package_addr, DEX_REG_EVENT_TYPE) => {
                let parsed: DexRegistrationEvent =
                    serde_json::from_value(event.data.clone()).unwrap();
                Self::DexRegistrationEvent(parsed)
            }
            s if s == format!("{}::{}", package_addr, SUBACCOUNT_CREATED_EVENT_TYPE) => {
                let parsed: SubaccountCreatedEvent =
                    serde_json::from_value(event.data.clone()).unwrap();
                Self::SubaccountCreatedEvent(parsed)
            }
            s if s == format!("{}::{}", package_addr, TRADE_EVENT_TYPE) => {
                let parsed: TradeEvent = serde_json::from_value(event.data.clone()).unwrap();
                Self::TradeEvent(parsed)
            }
            _ => Self::Unknown(()),
        }
    }
}

/// Extract all events from a transaction using the full Event struct
pub fn extract_events_from_transaction(transaction: &Transaction) -> Vec<Event> {
    Event::from_transaction(transaction)
}

/// Classify an event using the full KnownEvent enum
pub fn classify_event_full(event: &Event, package_addr: &str) -> KnownEvent {
    KnownEvent::from_event(event, package_addr)
}
