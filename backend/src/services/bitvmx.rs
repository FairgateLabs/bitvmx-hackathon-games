use crate::config::BitcoinConfig;
use crate::models::{P2PAddress, WalletBalance};
use crate::rpc::rpc_client::RpcClient;
use bitvmx_bitcoin_rpc::bitcoin_client::BitcoinClient;
use bitvmx_bitcoin_rpc::bitcoin_client::BitcoinClientApi;
use bitvmx_client::bitcoin::{Address, PublicKey, Transaction, Txid};
use bitvmx_client::bitcoin_coordinator::TransactionStatus;
use bitvmx_client::bitvmx_wallet::wallet::Destination;
use bitvmx_client::program::participant::P2PAddress as BitVMXP2PAddress;
use bitvmx_client::program::protocols::dispute;
use bitvmx_client::program::variables::VariableTypes;
use bitvmx_client::types::{IncomingBitVMXApiMessages, OutgoingBitVMXApiMessages};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::sleep;
use tracing::{debug, info, trace};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct BitVMXInfo {
    pub p2p_address: Option<P2PAddress>,
    pub pub_key: Option<String>,
    pub funding_key: Option<String>,
    pub wallet_address: Option<Address>,
}
#[derive(Debug, Clone)]
pub struct BitVMXService {
    pub bitvmx_info: Arc<RwLock<BitVMXInfo>>,
    pub bitcoin_config: BitcoinConfig,
    /// BitVMX RPC client
    pub rpc_client: Arc<RpcClient>,
}

impl BitVMXService {
    pub fn new(rpc_client: Arc<RpcClient>, bitcoin_config: BitcoinConfig) -> Self {
        Self {
            bitvmx_info: Arc::new(RwLock::new(BitVMXInfo {
                p2p_address: None,
                pub_key: None,
                funding_key: None,
                wallet_address: None,
            })),
            bitcoin_config: bitcoin_config.clone(),
            rpc_client,
        }
    }

    /// Get pub key
    pub async fn get_pub_key(&self) -> Result<Option<String>, anyhow::Error> {
        let bitvmx_info = self.bitvmx_info.read().await;
        Ok(bitvmx_info.pub_key.clone())
    }

    /// Get funding key
    pub async fn get_funding_key(&self) -> Result<Option<String>, anyhow::Error> {
        let bitvmx_info = self.bitvmx_info.read().await;
        Ok(bitvmx_info.funding_key.clone())
    }

    /// Get P2P address
    pub async fn get_p2p_address(&self) -> Result<Option<P2PAddress>, anyhow::Error> {
        let bitvmx_info = self.bitvmx_info.read().await;
        Ok(bitvmx_info.p2p_address.clone())
    }

    pub async fn get_wallet_address(&self) -> Result<Address, anyhow::Error> {
        let bitvmx_info = self.bitvmx_info.read().await;
        let wallet_address = bitvmx_info
            .wallet_address
            .clone()
            .ok_or(anyhow::anyhow!("Wallet address not found"))?;
        Ok(wallet_address)
    }

    /// Create aggregated key
    pub async fn create_agregated_key(
        &self,
        uuid: Uuid,
        participants: Vec<BitVMXP2PAddress>,
        participants_keys: Option<Vec<PublicKey>>,
        leader_idx: u16,
    ) -> Result<PublicKey, anyhow::Error> {
        trace!("Create aggregated key from BitVMX");
        let message =
            IncomingBitVMXApiMessages::SetupKey(uuid, participants, participants_keys, leader_idx);

        let response = self.rpc_client.send_request(message).await?;

        if let OutgoingBitVMXApiMessages::AggregatedPubkey(_uuid, aggregated_pubkey) = response {
            trace!("Obtained aggregated key: {:?}", aggregated_pubkey);
            Ok(aggregated_pubkey)
        } else {
            Err(anyhow::anyhow!(
                "Expected AggregatedPubkey response, got: {:?}",
                response
            ))
        }
    }

    /// Get aggregated key
    pub async fn aggregated_key(&self, aggregated_id: Uuid) -> Result<PublicKey, anyhow::Error> {
        trace!("Get aggregated key from BitVMX");
        let response = self
            .rpc_client
            .send_request(IncomingBitVMXApiMessages::GetAggregatedPubkey(
                aggregated_id,
            ))
            .await?;
        if let OutgoingBitVMXApiMessages::AggregatedPubkey(_uuid, aggregated_pubkey) = response {
            trace!("Obtained aggregated key: {:?}", aggregated_pubkey);
            Ok(aggregated_pubkey)
        } else if let OutgoingBitVMXApiMessages::AggregatedPubkeyNotReady(uuid) = response {
            Err(anyhow::anyhow!("Aggregated key not ready: {:?}", uuid))
        } else {
            Err(anyhow::anyhow!(
                "Expected AggregatedPubkey response, got: {:?}",
                response
            ))
        }
    }

    pub async fn wallet_balance(&self) -> Result<WalletBalance, anyhow::Error> {
        let address = self.get_wallet_address().await?;
        let response = self
            .rpc_client
            .send_request(IncomingBitVMXApiMessages::GetFundingBalance(Uuid::new_v4()))
            .await?;

        if let OutgoingBitVMXApiMessages::FundingBalance(_uuid, balance) = response {
            Ok(WalletBalance {
                address: address.to_string(),
                balance,
            })
        } else {
            Err(anyhow::anyhow!(
                "Expected Funding Address response, got: {:?}",
                response
            ))
        }
    }

    pub async fn send_funds(
        &self,
        destination: &Destination,
    ) -> Result<(Uuid, Txid), anyhow::Error> {
        let response = self
            .rpc_client
            .send_request(IncomingBitVMXApiMessages::SendFunds(
                Uuid::new_v4(),
                destination.clone(),
                None, // fee rate not needed for regtest
            ))
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send funds: {e:?}"))?;

        if let OutgoingBitVMXApiMessages::FundsSent(uuid, txid) = response {
            Ok((uuid, txid))
        } else {
            Err(anyhow::anyhow!(
                "Expected Funds Sent response, got: {:?}",
                response
            ))
        }
    }

    pub async fn send_funds_wait_confirmation(
        &self,
        destination: &Destination,
    ) -> Result<TransactionStatus, anyhow::Error> {
        let (uuid, _) = self.send_funds(destination).await?;
        let tx_status = self.wait_transaction_response(uuid).await?;
        Ok(tx_status)
    }

    pub async fn wait_transaction_response(
        &self,
        correlation_id: Uuid,
    ) -> Result<TransactionStatus, anyhow::Error> {
        debug!(
            "Waiting for transaction response for correlation id: {:?}",
            correlation_id
        );
        let response = self
            .rpc_client
            .wait_for_response(correlation_id.to_string())
            .await
            .map_err(|e| anyhow::anyhow!("Failed to wait for transaction response: {e:?}"))?;

        if let OutgoingBitVMXApiMessages::Transaction(_uuid, transaction_status, _) = response {
            info!(
                "Received transaction response for correlation id: {:?}",
                correlation_id
            );
            Ok(transaction_status)
        } else {
            Err(anyhow::anyhow!(
                "Expected Transaction response, got: {:?}",
                response
            ))
        }
    }

    pub async fn wait_transaction_by_name_response(
        &self,
        program_id: Uuid,
        name: &str,
    ) -> Result<Transaction, anyhow::Error> {
        debug!(
            "Waiting for transaction response for program id: {:?} and name: {:?}",
            program_id, name
        );
        let response = self
            .rpc_client
            .wait_for_response(RpcClient::tx_name_to_correlation_id(&program_id, name))
            .await?;
        if let OutgoingBitVMXApiMessages::TransactionInfo(_uuid, name, tx) = response {
            info!("Received transaction response for name: {:?}", name);
            Ok(tx)
        } else {
            Err(anyhow::anyhow!(
                "Expected TransactionInfo response, got: {:?}",
                response
            ))
        }
    }

    pub async fn get_transaction(&self, txid: String) -> Result<TransactionStatus, anyhow::Error> {
        let response = self
            .rpc_client
            .send_request(IncomingBitVMXApiMessages::GetTransaction(
                Uuid::new_v4(),
                Txid::from_str(&txid)?,
            ))
            .await?;

        if let OutgoingBitVMXApiMessages::Transaction(_uuid, transaction_status, _) = response {
            Ok(transaction_status)
        } else {
            Err(anyhow::anyhow!(
                "Expected Transaction response, got: {:?}",
                response
            ))
        }
    }

    pub async fn set_variable(
        &self,
        program_id: Uuid,
        key: &str,
        value: VariableTypes,
    ) -> Result<(), anyhow::Error> {
        self.rpc_client
            .send_fire_and_forget(IncomingBitVMXApiMessages::SetVar(
                program_id,
                key.to_string(),
                value,
            ))
            .await?;

        Ok(())
    }

    pub async fn program_setup(
        &self,
        program_id: Uuid,
        program_type: &str,
        participants: Vec<BitVMXP2PAddress>,
        leader_idx: u16,
    ) -> Result<Uuid, anyhow::Error> {
        let response = self
            .rpc_client
            .send_request(IncomingBitVMXApiMessages::Setup(
                program_id,
                program_type.to_string(),
                participants,
                leader_idx,
            ))
            .await?;

        if let OutgoingBitVMXApiMessages::SetupCompleted(uuid) = response {
            if uuid != program_id {
                return Err(anyhow::anyhow!(
                    "Expected SetupCompleted response with program ID: {:?}, got: {:?}",
                    program_id,
                    uuid
                ));
            }
        } else {
            return Err(anyhow::anyhow!(
                "Expected SetupCompleted response, got: {:?}",
                response
            ));
        }

        Ok(program_id)
    }

    pub fn protocol_cost(&self) -> u64 {
        bitvmx_client::program::protocols::dispute::protocol_cost()
    }

    pub async fn start_challenge(&self, program_id: Uuid) -> Result<Transaction, anyhow::Error> {
        // Dispatch the start challenge transaction
        let response = self
            .rpc_client
            .send_request(IncomingBitVMXApiMessages::DispatchTransactionName(
                program_id,
                dispute::START_CH.to_string(),
            ))
            .await?;

        if let OutgoingBitVMXApiMessages::TransactionInfo(_uuid, name, tx) = response {
            if name != dispute::START_CH.to_string() {
                return Err(anyhow::anyhow!(
                    "Expected Dispute Started response with name: {:?}, got: {:?}",
                    dispute::START_CH.to_string(),
                    name
                ));
            }
            Ok(tx)
        } else {
            Err(anyhow::anyhow!(
                "Expected Dispute Started response, got: {:?}",
                response
            ))
        }
    }

    /// Get the name of the input transaction
    pub fn program_input_name(index: u32) -> String {
        format!("program_input_{index}")
    }

    /// Set the program input
    pub async fn set_program_input(
        &self,
        program_id: Uuid,
        index: u32,
        value: Vec<u8>,
    ) -> Result<(), anyhow::Error> {
        self.set_variable(
            program_id,
            &Self::program_input_name(index),
            VariableTypes::Input(value),
        )
        .await
    }

    /// Send the transaction by name
    pub async fn send_transaction_by_name(
        &self,
        program_id: Uuid,
        tx_name: String,
    ) -> Result<(Transaction, String), anyhow::Error> {
        // Dispatch the transaction by name
        let response = self
            .rpc_client
            .send_request(IncomingBitVMXApiMessages::DispatchTransactionName(
                program_id,
                tx_name.clone(),
            ))
            .await?;

        if let OutgoingBitVMXApiMessages::TransactionInfo(_uuid, name, tx) = response {
            if name != tx_name {
                return Err(anyhow::anyhow!(
                    "Expected Transaction Info response with name: {:?}, got: {:?}",
                    tx_name,
                    name
                ));
            }
            Ok((tx, name))
        } else {
            Err(anyhow::anyhow!(
                "Expected Transaction Info response, got: {:?}",
                response
            ))
        }
    }

    /// Get the name of the input transaction
    pub fn input_tx_name(index: u32) -> String {
        format!("INPUT_{index}")
    }

    /// Send the challenge input transaction
    pub async fn send_challenge_input(
        &self,
        program_id: Uuid,
        index: u32,
    ) -> Result<(Transaction, String), anyhow::Error> {
        let tx_name = Self::input_tx_name(index);
        self.send_transaction_by_name(program_id, tx_name).await
    }

    pub async fn get_funding_address(&self) -> Result<Address, anyhow::Error> {
        let response = self
            .rpc_client
            .send_request(IncomingBitVMXApiMessages::GetFundingAddress(Uuid::new_v4()))
            .await?;
        if let OutgoingBitVMXApiMessages::FundingAddress(_uuid, address) = response {
            Ok(address.assume_checked())
        } else {
            Err(anyhow::anyhow!(
                "Expected Funding Address response, got: {:?}",
                response
            ))
        }
    }

    pub async fn get_funding_balance(&self) -> Result<u64, anyhow::Error> {
        let response = self
            .rpc_client
            .send_request(IncomingBitVMXApiMessages::GetFundingBalance(Uuid::new_v4()))
            .await?;

        if let OutgoingBitVMXApiMessages::FundingBalance(_uuid, balance) = response {
            Ok(balance)
        } else if let OutgoingBitVMXApiMessages::WalletNotReady(uuid) = response {
            Err(anyhow::anyhow!(
                "Wallet not ready correlation id: {:?}",
                uuid
            ))
        } else {
            Err(anyhow::anyhow!(
                "Expected Funding Balance response, got: {:?}",
                response
            ))
        }
    }

    pub async fn get_or_generate_pub_key(&self, is_new: bool) -> Result<PublicKey, anyhow::Error> {
        let pub_key_id = Uuid::new_v4();
        let response = self
            .rpc_client
            .send_request(IncomingBitVMXApiMessages::GetPubKey(pub_key_id, is_new))
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get pub key: {e:?}"))?;

        if let OutgoingBitVMXApiMessages::PubKey(_uuid, pub_key) = response {
            Ok(pub_key)
        } else {
            return Err(anyhow::anyhow!(
                "Expected Operator PubKey response, got: {:?}",
                response
            ));
        }
    }

    // ----- Start internal methods -----

    /// Update P2P address
    async fn set_wallet_address(&self) -> Result<(), anyhow::Error> {
        let wallet_address: Address = self.get_funding_address().await?;
        self.bitvmx_info.write().await.wallet_address = Some(wallet_address.clone());

        debug!("Adding funds for wallet address: {:?}", wallet_address);

        let bitcoin_config = self.bitcoin_config.clone();
        // run a blocking routine without blocking the runtime
        // fund the wallet address with 2 blocks coinbase (mine 100 blocks for maturity)
        tokio::task::spawn_blocking(move || {
            // create the bitcoin client
            let bitcoin_client = BitcoinClient::new(
                &bitcoin_config.url,
                &bitcoin_config.username,
                &bitcoin_config.password,
            )
            .unwrap();

            // each block gives a 50 BTC reward
            bitcoin_client
                .mine_blocks_to_address(2, &wallet_address)
                .unwrap();

            // mine 100 blocks for maturity
            bitcoin_client.mine_blocks(100).unwrap();
        })
        .await
        .map_err(|e| anyhow::anyhow!("Failed to mine blocks for funding wallet address: {e:?}"))?;

        debug!("Mined 2 blocks and 100 blocks, waiting for bitvmx to process them");
        // wait for bitvmx to process the blocks
        sleep(Duration::from_secs(2)).await;
        debug!("Waited 2 seconds");

        let balance = self.get_funding_balance().await?;
        debug!("Funding balance: {:?}", balance);

        if balance < 100_000_000 {
            return Err(anyhow::anyhow!("Funding balance is less than 1 BTC"));
        }

        trace!("Updated wallet address in store");
        Ok(())
    }

    /// Update P2P address
    async fn set_p2p_address(&self) -> Result<(), anyhow::Error> {
        // Set P2P address
        let response = self
            .rpc_client
            .send_request(IncomingBitVMXApiMessages::GetCommInfo())
            .await?;
        if let OutgoingBitVMXApiMessages::CommInfo(comm_info) = response {
            self.bitvmx_info.write().await.p2p_address = Some(P2PAddress {
                address: comm_info.address.clone(),
                peer_id: comm_info.peer_id.to_string(),
            });
        } else {
            return Err(anyhow::anyhow!(
                "Expected Comm Info response, got: {:?}",
                response
            ));
        }
        trace!("Updated P2P address in store");
        Ok(())
    }

    /// Update pub key
    async fn set_pub_key(&self) -> Result<(), anyhow::Error> {
        debug!("Create operator key from BitVMX");
        let pub_key = self.get_or_generate_pub_key(true).await?;
        info!("Operator key: {:?}", pub_key);
        self.bitvmx_info.write().await.pub_key = Some(pub_key.to_string());

        trace!("Updated pub key in store");
        Ok(())
    }

    /// Update funding key
    async fn set_funding_key(&self) -> Result<(), anyhow::Error> {
        debug!("Create funding key for speedups from BitVMX");
        let funding_pubkey = self.get_or_generate_pub_key(true).await?;
        info!("Funding key: {:?}", funding_pubkey);
        self.bitvmx_info.write().await.funding_key = Some(funding_pubkey.to_string());
        trace!("Updated funding key in store");

        // Send 1 BTC to the funding key
        let amount = 100_000_000; // 1 BTC
        let (uuid, _txid) = self
            .send_funds(&Destination::P2WPKH(funding_pubkey, amount))
            .await?;

        // mine 1 block to ensure it's confirmed
        // run a blocking routine without blocking the runtime
        let bitcoin_config = self.bitcoin_config.clone();
        tokio::task::spawn_blocking(move || {
            let bitcoin_client = BitcoinClient::new(
                &bitcoin_config.url,
                &bitcoin_config.username,
                &bitcoin_config.password,
            )
            .map_err(|e| anyhow::anyhow!("Failed to create bitcoin client: {e:?}"))
            .unwrap();
            bitcoin_client.mine_blocks(1).unwrap();
        })
        .await
        .map_err(|e| anyhow::anyhow!("Failed to mine 1 block for confirmation: {e:?}"))?;

        // Wait for the transaction confirmation reponse to use the utxo
        let tx_status = self.wait_transaction_response(uuid).await?;

        self.rpc_client
            .send_request(IncomingBitVMXApiMessages::SetFundingUtxo(
                bitvmx_client::protocol_builder::types::Utxo {
                    txid: tx_status.tx_id,
                    vout: 0,
                    amount: amount,
                    pub_key: funding_pubkey,
                },
            ))
            .await
            .map_err(|e| anyhow::anyhow!("Failed to set funding utxo: {e:?}"))?;

        trace!("Updated funding key in store");
        Ok(())
    }

    /// Setup BitVMX
    pub async fn initial_setup(&self) -> Result<(), anyhow::Error> {
        debug!("Get BitVMX info and initial keys setup");

        self.set_p2p_address().await?;

        // Set wallet address
        self.set_wallet_address().await?;

        // If keys do not exist, setup keys
        if self.get_pub_key().await?.is_none() {
            debug!("No keys found, creating them");
            // Set operator pub key
            self.set_pub_key().await?;

            // Set funding key
            self.set_funding_key().await?;
        } else {
            return Err(anyhow::anyhow!("Keys already exist!!! cannot setup"));
        }

        Ok(())
    }

    // ----- End internal methods -----
}
