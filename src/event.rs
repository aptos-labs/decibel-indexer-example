use aptos_indexer_processor_sdk::{
    aptos_protos::transaction::v1::{transaction::TxnData, Event as EventPB, Transaction},
    utils::convert::{standardize_address, truncate_str},
};

use crate::types::timestamp_to_unix_millis;

// p99 currently is 303 so using 300 as a safe max length
const EVENT_TYPE_MAX_LENGTH: usize = 300;

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
        let t: &str = event.type_str.as_ref();
        Event {
            account_address: standardize_address(
                event.key.as_ref().unwrap().account_address.as_str(),
            ),
            creation_number: event.key.as_ref().unwrap().creation_number as i64,
            sequence_number: event.sequence_number as i64,
            transaction_version,
            transaction_block_height,
            transaction_unix_ms,
            type_: t.to_string(),
            data: serde_json::from_str(event.data.as_str()).unwrap_or(serde_json::Value::Null),
            event_index,
            indexed_type: truncate_str(t, EVENT_TYPE_MAX_LENGTH),
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
