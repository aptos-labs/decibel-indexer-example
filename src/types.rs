use aptos_indexer_processor_sdk::utils::convert::standardize_address;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct OptionType<T> {
    vec: Vec<T>,
}

#[allow(dead_code)]
impl<T> OptionType<T> {
    pub fn is_some(&self) -> bool {
        !self.vec.is_empty()
    }

    pub fn first(&self) -> Option<&T> {
        self.vec.first()
    }
}

#[allow(dead_code)]
impl OptionType<String> {
    pub fn first_as_u64(&self) -> Option<u64> {
        self.first()?.parse::<u64>().ok()
    }

    pub fn unwrap_or_empty(&self) -> String {
        self.first().map(|s| s.to_string()).unwrap_or_default()
    }
}

pub fn timestamp_to_unix_millis(
    timestamp: &aptos_indexer_processor_sdk::aptos_protos::util::timestamp::Timestamp,
) -> i64 {
    (timestamp.seconds * 1000) + (timestamp.nanos / 1_000_000) as i64
}
