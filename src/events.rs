use aptos_indexer_processor_sdk::aptos_protos::transaction::v1::{
    transaction::TxnData, Transaction,
};
use serde::Deserialize;

// Event type constants - update these to match your actual package events
pub const ORDER_EVENT_TYPE: &str = "market_types::OrderEvent";
pub const TRADE_EVENT_TYPE: &str = "perp_positions::TradeEvent";
pub const MARKET_REG_EVENT_TYPE: &str = "perp_engine::MarketRegistrationEvent";

// Simple event structures based on etna patterns
#[derive(Debug, Deserialize, Clone)]
pub struct OrderEvent {
    pub user: String,
    pub market: String,
    // Add more fields as needed based on your actual event structure
}

#[derive(Debug, Deserialize, Clone)]
pub struct TradeEvent {
    pub account: String,
    pub market: String,
    // Add more fields as needed based on your actual event structure
}

#[derive(Debug, Deserialize, Clone)]
pub struct MarketRegistrationEvent {
    pub market: String,
    // Add more fields as needed based on your actual event structure
}

#[derive(Debug, Clone)]
pub struct SimpleEvent {
    pub type_: String,
    pub data: serde_json::Value,
}

pub enum EventType {
    OrderEvent(OrderEvent),
    TradeEvent(TradeEvent),
    MarketRegistrationEvent(MarketRegistrationEvent),
}

/// Extract all events from a transaction
pub fn extract_events_from_transaction(transaction: &Transaction) -> Vec<SimpleEvent> {
    let txn_data = match &transaction.txn_data {
        Some(data) => data,
        None => return vec![],
    };

    let raw_events = match txn_data {
        TxnData::Genesis(tx_inner) => &tx_inner.events,
        TxnData::User(tx_inner) => &tx_inner.events,
        _ => return vec![],
    };

    raw_events
        .iter()
        .filter_map(|event| {
            let event_type = event.type_str.as_str();
            let data = serde_json::from_str(&event.data).ok()?;

            Some(SimpleEvent {
                type_: event_type.to_string(),
                data,
            })
        })
        .collect()
}

/// Classify an event based on its type and package address
pub fn classify_event(event: &SimpleEvent, package_addr: &str) -> Option<EventType> {
    match event.type_.as_str() {
        s if s == format!("{}::{}", package_addr, ORDER_EVENT_TYPE) => {
            serde_json::from_value(event.data.clone())
                .ok()
                .map(EventType::OrderEvent)
        }
        s if s == format!("{}::{}", package_addr, TRADE_EVENT_TYPE) => {
            serde_json::from_value(event.data.clone())
                .ok()
                .map(EventType::TradeEvent)
        }
        s if s == format!("{}::{}", package_addr, MARKET_REG_EVENT_TYPE) => {
            serde_json::from_value(event.data.clone())
                .ok()
                .map(EventType::MarketRegistrationEvent)
        }
        _ => None,
    }
}
