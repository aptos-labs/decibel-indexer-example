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
use events::{classify_event, extract_events_from_transaction, EventType};
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
    /// Aptos node GRPC endpoint
    pub indexer_grpc_data_service_address: String,
    /// Starting version for indexing (optional, defaults to 0)
    pub starting_version: Option<u64>,
    /// Ending version for indexing (optional, runs indefinitely if not set)
    pub ending_version: Option<u64>,
    /// Package address to filter events for
    pub package_address: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            indexer_grpc_data_service_address: "https://grpc.mainnet.aptoslabs.com:443".to_string(),
            starting_version: Some(0),
            ending_version: None,
            package_address: "0x1".to_string(),
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

    info!("Loading configuration from {:?}", args.config);
    let config_content = std::fs::read_to_string(&args.config)?;
    let config: Config = serde_yaml::from_str(&config_content).unwrap_or_else(|_| {
        info!("Failed to parse config, using defaults");
        Config::default()
    });

    info!("Starting Decibel Events Processor");
    info!("Package address: {}", config.package_address);
    info!(
        "GRPC endpoint: {}",
        config.indexer_grpc_data_service_address
    );

    if let Some(start) = config.starting_version {
        info!("Starting from version: {}", start);
    }

    if let Some(end) = config.ending_version {
        info!("Ending at version: {}", end);
    }

    run_processor(config).await
}

async fn run_processor(config: Config) -> Result<()> {
    // Create transaction stream configuration
    let transaction_stream_config = TransactionStreamConfig {
        indexer_grpc_data_service_address: config.indexer_grpc_data_service_address.parse()?,
        starting_version: config.starting_version,
        request_ending_version: config.ending_version,
        auth_token: "".to_string(),
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

    info!("Building processor pipeline");

    // Create processor steps
    let transaction_stream = TransactionStreamStep::new(transaction_stream_config).await?;

    // Build the processor pipeline - much simpler!
    let (_, buffer_receiver) =
        ProcessorBuilder::new_with_inputless_first_step(transaction_stream.into_runnable_step())
            .connect_to(processor.into_runnable_step(), 10)
            .end_and_return_output_receiver(10);

    info!("Starting transaction processing");

    // Process transactions
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
    let mut total_events = 0;
    let mut order_events = 0;
    let mut trade_events = 0;
    let mut market_events = 0;
    let mut unknown_events = 0;

    for transaction in &transactions {
        let events = extract_events_from_transaction(transaction);

        for event in events {
            total_events += 1;

            match classify_event(&event, package_addr) {
                Some(EventType::OrderEvent(order)) => {
                    order_events += 1;
                    info!(user = order.user, market = order.market, "Found OrderEvent");
                }
                Some(EventType::TradeEvent(trade)) => {
                    trade_events += 1;
                    info!(
                        account = trade.account,
                        market = trade.market,
                        "Found TradeEvent"
                    );
                }
                Some(EventType::MarketRegistrationEvent(market)) => {
                    market_events += 1;
                    info!(market = market.market, "Found MarketRegistrationEvent");
                }
                None => {
                    unknown_events += 1;
                    info!(event_type = event.type_, "Unknown event type");
                }
            }
        }
    }

    if total_events > 0 {
        info!(
            total_events = total_events,
            order_events = order_events,
            trade_events = trade_events,
            market_events = market_events,
            unknown_events = unknown_events,
            "Processed event batch"
        );
    }

    Ok(vec![(); transactions.len()])
}
