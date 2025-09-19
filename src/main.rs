mod event_parsing_utils;
mod events;

use anyhow::Result;
use aptos_indexer_processor_sdk::{
    aptos_indexer_transaction_stream::TransactionStreamConfig,
    aptos_protos::transaction::v1::Transaction,
    builder::ProcessorBuilder,
    common_steps::TransactionStreamStep,
    traits::{async_step::AsyncRunType, AsyncStep, IntoRunnableStep, NamedStep, Processable},
    types::transaction_context::TransactionContext,
    utils::errors::ProcessorError,
};
use async_trait::async_trait;
use clap::Parser;
use event_parsing_utils::{classify_event_full, extract_events_from_transaction, KnownEvent};
use std::path::PathBuf;
use tracing::{info, Level};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to configuration file
    #[arg(short, long)]
    config: PathBuf,
}

#[derive(serde::Deserialize)]
struct Config {
    /// Aptos node gRPC endpoint
    pub indexer_grpc_data_service_address: String,
    /// Starting version for indexing (optional, defaults to 0)
    pub starting_version: Option<u64>,
    /// Ending version for indexing (optional, runs indefinitely if not set)
    pub ending_version: Option<u64>,
    /// Package address to filter events for
    pub package_address: String,
    /// Auth token for GRPC endpoint (required for staging/private endpoints)
    pub auth_token: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            indexer_grpc_data_service_address: "https://grpc.netna.staging.aptoslabs.com:443"
                .to_string(),
            starting_version: Some(0),
            ending_version: None,
            package_address: "0xb8a5788314451ce4d2fbbad32e1bad88d4184b73943b7fe5166eab93cf1a5a95"
                .to_string(),
            auth_token: None,
        }
    }
}

// Simple processor that processes transactions directly
pub struct DecibelProcessor {
    pub package_addr: String,
}

impl DecibelProcessor {
    pub fn new(package_addr: String) -> Self {
        Self { package_addr }
    }
}

#[async_trait]
impl Processable for DecibelProcessor {
    type Input = Vec<Transaction>;
    type Output = ();
    type RunType = AsyncRunType;

    async fn process(
        &mut self,
        transactions: TransactionContext<Vec<Transaction>>,
    ) -> Result<Option<TransactionContext<Self::Output>>, ProcessorError> {
        match process_transactions(transactions.data, &self.package_addr) {
            Ok(_) => Ok(Some(TransactionContext {
                data: (),
                metadata: transactions.metadata,
            })),
            Err(e) => Err(ProcessorError::ProcessError {
                message: format!("Failed to process transactions: {}", e),
            }),
        }
    }
}

impl AsyncStep for DecibelProcessor {}

impl NamedStep for DecibelProcessor {
    fn name(&self) -> String {
        "DecibelProcessor".to_string()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .init();

    let args = Args::parse();
    let config_content = std::fs::read_to_string(&args.config)?;
    let config: Config = serde_yaml::from_str(&config_content).unwrap_or_else(|_| {
        info!("Failed to parse config, using defaults");
        Config::default()
    });

    run_processor(config).await
}

async fn run_processor(config: Config) -> Result<()> {
    // Create transaction stream configuration
    let transaction_stream_config = TransactionStreamConfig {
        indexer_grpc_data_service_address: config.indexer_grpc_data_service_address.parse()?,
        starting_version: config.starting_version,
        request_ending_version: config.ending_version,
        auth_token: config.auth_token.unwrap_or_default(),
        request_name_header: "decibel-indexer-example".to_string(),
        additional_headers: Default::default(),
        indexer_grpc_http2_ping_interval_secs: 60,
        indexer_grpc_http2_ping_timeout_secs: 10,
        indexer_grpc_reconnection_timeout_secs: 10,
        indexer_grpc_reconnection_max_retries: 5,
        indexer_grpc_response_item_timeout_secs: 30,
        transaction_filter: None,
    };

    let processor = DecibelProcessor::new(config.package_address.clone());
    let transaction_stream = TransactionStreamStep::new(transaction_stream_config).await?;

    let (_, buffer_receiver) =
        ProcessorBuilder::new_with_inputless_first_step(transaction_stream.into_runnable_step())
            .connect_to(processor.into_runnable_step(), 10)
            .end_and_return_output_receiver(10);

    loop {
        match buffer_receiver.recv().await {
            Ok(txn_context) => {
                info!(
                    "Processed transactions from version {} to {}",
                    txn_context.metadata.start_version, txn_context.metadata.end_version
                );
            }
            Err(_) => {
                info!("Channel is closed, shutting down");
                break;
            }
        }
    }

    Ok(())
}

fn process_transactions(transactions: Vec<Transaction>, package_addr: &str) -> Result<Vec<()>> {
    // Add business logic to handle events.
    for transaction in &transactions {
        let events = extract_events_from_transaction(transaction);
        for event in events {
            match classify_event_full(&event, package_addr) {
                KnownEvent::OrderEvent(_order) => {}
                KnownEvent::TradeEvent(_trade) => {}
                KnownEvent::MarketRegistrationEvent(_market) => {}
                KnownEvent::PositionUpdateEvent(_position) => {}
                KnownEvent::PriceUpdateEvent(_price) => {}
                KnownEvent::CollateralBalanceChangeEvent(_collateral) => {}
                KnownEvent::DexRegistrationEvent(_dex) => {}
                KnownEvent::SubaccountCreatedEvent(_subaccount) => {}
                KnownEvent::Unknown(_) => {}
            }
        }
    }

    Ok(vec![(); transactions.len()])
}
