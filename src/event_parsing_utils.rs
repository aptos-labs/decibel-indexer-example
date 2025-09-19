use aptos_indexer_processor_sdk::{
    aptos_protos::transaction::v1::{transaction::TxnData, Event as EventPB, Transaction},
    utils::convert::{standardize_address, truncate_str},
};

use crate::events::*;

// p99 currently is 303 so using 300 as a safe max length
const EVENT_TYPE_MAX_LENGTH: usize = 300;

const ORDER_EVENT_TYPE: &str = "market_types::OrderEvent";
const MARKET_REG_EVENT_TYPE: &str = "perp_engine::MarketRegistrationEvent";
const POSITION_UPDATE_EVENT_TYPE: &str = "perp_positions::PositionUpdateEvent";
const PRICE_UPDATE_EVENT_TYPE: &str = "price_management::PriceUpdateEvent";
const COLLATERAL_BALANCE_CHANGE_EVENT_TYPE: &str =
    "collateral_balance_sheet::CollateralBalanceChangeEvent";
const DEX_REG_EVENT_TYPE: &str = "perp_engine::DexRegistrationEvent";
const SUBACCOUNT_CREATED_EVENT_TYPE: &str = "dex_accounts::SubaccountCreatedEvent";
const TRADE_EVENT_TYPE: &str = "perp_positions::TradeEvent";

// KnownEvent enum with all supported decibel event types.
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
                let parsed: OrderEvent = serde_json::from_value(event.data.clone()).unwrap();
                Self::OrderEvent(parsed)
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

pub fn timestamp_to_unix_millis(
    timestamp: &aptos_indexer_processor_sdk::aptos_protos::util::timestamp::Timestamp,
) -> i64 {
    (timestamp.seconds * 1000) + (timestamp.nanos / 1_000_000) as i64
}

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

pub fn extract_events_from_transaction(transaction: &Transaction) -> Vec<Event> {
    Event::from_transaction(transaction)
}

pub fn classify_event_full(event: &Event, package_addr: &str) -> KnownEvent {
    KnownEvent::from_event(event, package_addr)
}
