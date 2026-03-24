mod event;
mod event_router;
mod events;
mod types;

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
use event::extract_events_from_transaction;
use event_router::{EventTypeKeys, KnownEvent};
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
    /// Orderbook address (orders/bulk-orders use this, not package_address)
    pub orderbook_address: String,
    /// Auth token for gRPC endpoint (required for mainnet, optional for testnet)
    pub auth_token: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            indexer_grpc_data_service_address: "https://grpc.testnet.aptoslabs.com:443".to_string(),
            starting_version: Some(0),
            ending_version: None,
            package_address:
                "0xe7da2794b1d8af76532ed95f38bfdf1136abfd8ea3a240189971988a83101b7f".to_string(),
            orderbook_address:
                "0xe7da2794b1d8af76532ed95f38bfdf1136abfd8ea3a240189971988a83101b7f".to_string(),
            auth_token: None,
        }
    }
}

pub struct DecibelProcessor {
    keys: EventTypeKeys,
}

impl DecibelProcessor {
    pub fn new(package_addr: &str, orderbook_addr: &str) -> Self {
        Self {
            keys: EventTypeKeys::new(package_addr, orderbook_addr),
        }
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
        match process_transactions(transactions.data, &self.keys) {
            Ok(()) => Ok(Some(TransactionContext {
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

    let processor = DecibelProcessor::new(&config.package_address, &config.orderbook_address);
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

fn process_transactions(
    transactions: Vec<Transaction>,
    keys: &EventTypeKeys,
) -> Result<()> {
    // Add your business logic to handle events here.
    // Each match arm gives you a parsed, typed event — plug in your storage, analytics, etc.
    for transaction in &transactions {
        let events = extract_events_from_transaction(transaction);
        for event in events {
            match KnownEvent::from_event(&event, keys) {
                KnownEvent::OrderEvent(order) => {
                    info!(
                        "Order: id={} user={} status={:?} market={}",
                        order.order_id, order.user, order.status, order.market
                    );
                }
                KnownEvent::BulkOrderUpdatedEvent(bulk) => {
                    info!(
                        "BulkOrderUpdated: market={} account={} seq={}",
                        bulk.market, bulk.account, bulk.sequence_number
                    );
                }
                KnownEvent::BulkOrderFillEvent(fill) => {
                    info!(
                        "BulkOrderFill: market={} account={} price={} fill_size={}",
                        fill.market, fill.account, fill.price, fill.fill_size
                    );
                }
                KnownEvent::BulkOrderRejectedEvent(rejected) => {
                    info!(
                        "BulkOrderRejected: market={} account={} seq={}",
                        rejected.market, rejected.account, rejected.sequence_number
                    );
                }
                KnownEvent::MarketUpdateEvent(market) => {
                    info!(
                        "MarketUpdate: name={} sz_decimals={} max_leverage={}",
                        market.name, market.sz_decimals, market.max_leverage
                    );
                }
                KnownEvent::PositionUpdateEvent(position) => {
                    info!(
                        "Position: user={} is_long={} size={} leverage={}",
                        position.user, position.is_long, position.size, position.user_leverage
                    );
                }
                KnownEvent::PriceUpdateEvent(price) => {
                    info!(
                        "Price: oracle={} mark={} impact_ask={} impact_bid={}",
                        price.oracle_px, price.mark_px, price.impact_ask_px, price.impact_bid_px
                    );
                }
                KnownEvent::CollateralBalanceChangeEvent(collateral) => {
                    info!(
                        "CollateralChange: delta={} change_type={:?}",
                        collateral.delta, collateral.change_type
                    );
                }
                KnownEvent::DexRegistrationEvent(dex) => {
                    info!(
                        "DexRegistration: decimals={}",
                        dex.collateral_balance_decimals
                    );
                }
                KnownEvent::SubaccountCreatedEvent(sub) => {
                    info!(
                        "SubaccountCreated: subaccount={} owner={} is_primary={:?}",
                        sub.subaccount, sub.owner, sub.is_primary
                    );
                }
                KnownEvent::SubaccountActiveChangedEvent(sub) => {
                    info!(
                        "SubaccountActiveChanged: subaccount={} is_active={:?}",
                        sub.subaccount, sub.is_active
                    );
                }
                KnownEvent::DelegationChangedEvent(delegation) => {
                    info!(
                        "DelegationChanged: subaccount={} delegated_account={}",
                        delegation.subaccount, delegation.delegated_account
                    );
                }
                KnownEvent::TradeEvent(trade) => {
                    info!(
                        "Trade: account={} action={:?} source={:?} size={} price={} pnl={} fee={}",
                        trade.account,
                        trade.action,
                        trade.source,
                        trade.size,
                        trade.price,
                        trade.realized_pnl,
                        trade.fee
                    );
                }
                KnownEvent::TwapEvent(twap) => {
                    info!(
                        "Twap: account={} is_buy={} status={:?}",
                        twap.account, twap.is_buy, twap.status
                    );
                }
                KnownEvent::OpenInterestUpdateEvent(oi) => {
                    info!("OpenInterest: current={}", oi.current_open_interest);
                }
                KnownEvent::VaultCreatedEvent(vault) => {
                    info!(
                        "VaultCreated: name={} creator={}",
                        vault.vault_name, vault.creator
                    );
                }
                KnownEvent::VaultActivatedEvent(vault) => {
                    info!("VaultActivated: shares={} nav={}", vault.num_shares, vault.nav);
                }
                KnownEvent::VaultAdminChangedEvent(vault) => {
                    info!(
                        "VaultAdminChanged: admin={} new_admin={}",
                        vault.admin, vault.new_admin
                    );
                }
                KnownEvent::ContributionEvent(contribution) => {
                    info!(
                        "Contribution: user={} assets={} shares={}",
                        contribution.user,
                        contribution.assets_contributed,
                        contribution.shares_received
                    );
                }
                KnownEvent::RedeemptionInitiatedEvent(redemption) => {
                    info!(
                        "RedemptionInitiated: user={} shares={}",
                        redemption.user, redemption.shares_to_redeem
                    );
                }
                KnownEvent::RedeemptionSettledEvent(redemption) => {
                    info!(
                        "RedemptionSettled: user={} shares={} assets={}",
                        redemption.user, redemption.shares_redeemed, redemption.assets_received
                    );
                }
                KnownEvent::FeeDistributionEvent(fee) => {
                    info!(
                        "FeeDistribution: recipient={} fee_amount={}",
                        fee.fee_recipient, fee.current_fee_amount
                    );
                }
                KnownEvent::GlobalVaultConfigUpdatedEvent(config) => {
                    info!(
                        "GlobalVaultConfigUpdated: max_fee_bps={}",
                        config.max_fee_bps
                    );
                }
                KnownEvent::MarketModeChangedEvent(mode) => {
                    info!("MarketModeChanged: mode={:?}", mode.mode);
                }
                KnownEvent::LiquidationEvent(liq) => {
                    info!(
                        "Liquidation (deprecated): user={} type={:?}",
                        liq.user, liq.liquidation_type
                    );
                }
                KnownEvent::MarginCallLogEvent(mcl) => {
                    info!(
                        "MarginCallLog: account={} result={:?}",
                        mcl.account, mcl.result
                    );
                }
                KnownEvent::AckEvent(ack) => {
                    info!("Ack: phase={:?} accounts={}", ack.ack_phase, ack.accounts.len());
                }
                KnownEvent::UsdcMintEvent(mint) => {
                    info!("UsdcMint: account={} amount={}", mint.account, mint.amount);
                }
                KnownEvent::ReferralCodeRegisteredEvent(referral) => {
                    let code = match &referral {
                        events::ReferralCodeRegisteredEvent::V1(inner) => &inner.referral_code,
                    };
                    info!("ReferralCodeRegistered: code={}", code);
                }
                KnownEvent::ReferralRegisteredEvent(referral) => {
                    let account = match &referral {
                        events::ReferralRegisteredEvent::V1(inner) => &inner.referree_addr,
                    };
                    info!("ReferralRegistered: account={}", account);
                }
                KnownEvent::AffiliateRegisteredEvent(affiliate) => {
                    let account = match &affiliate {
                        events::AffiliateRegisteredEvent::V1(inner) => &inner.affiliate_addr,
                    };
                    info!("AffiliateRegistered: account={}", account);
                }
                KnownEvent::Unknown(_) => {}
            }
        }
    }

    Ok(())
}
