use crate::config::BitcoinConfig;
use crate::models::{P2PAddress, WalletBalance};
use crate::rpc::rpc_client::RpcClient;
use crate::utils;
use bitvmx_bitcoin_rpc::bitcoin_client::BitcoinClient;
use bitvmx_bitcoin_rpc::bitcoin_client::BitcoinClientApi;
use bitvmx_client::bitcoin::{Address, PublicKey, Txid};
use bitvmx_client::bitcoin_coordinator::TransactionStatus;
use bitvmx_client::program::participant::P2PAddress as BitVMXP2PAddress;
use bitvmx_client::program::variables::{PartialUtxo, VariableTypes};
use bitvmx_client::protocol_builder::types::OutputType;
use bitvmx_client::types::{Destination, IncomingBitVMXApiMessages, OutgoingBitVMXApiMessages};
use std::str::FromStr;
use std::sync::Arc;
use tracing::{debug, error, trace};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct BitVMXService {
    pub p2p_address: Option<P2PAddress>,
    pub pub_key: Option<String>,
    pub funding_key: Option<String>,
    pub wallet_address: Option<Address>,
    pub bitcoin_config: BitcoinConfig,

    /// BitVMX RPC client
    pub rpc_client: Arc<RpcClient>,
}

impl BitVMXService {
    pub fn new(rpc_client: Arc<RpcClient>, bitcoin_config: BitcoinConfig) -> Self {
        Self {
            p2p_address: None,
            pub_key: None,
            funding_key: None,
            wallet_address: None,
            bitcoin_config: bitcoin_config.clone(),
            rpc_client,
        }
    }

    /// Get pub key
    pub fn get_pub_key(&self) -> Option<String> {
        self.pub_key.clone()
    }

    /// Get funding key
    pub fn get_funding_key(&self) -> Option<String> {
        self.funding_key.clone()
    }

    /// Get P2P address
    pub fn get_p2p_address(&self) -> Option<P2PAddress> {
        self.p2p_address.clone()
    }

    pub fn get_wallet_address(&self) -> Result<&Address, anyhow::Error> {
        self.wallet_address
            .as_ref()
            .ok_or(anyhow::anyhow!("Wallet address not found"))?;
        Ok(self.wallet_address.as_ref().unwrap())
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

    /// Create aggregated key with callback - non-blocking version
    /// This method sends the request and executes the callback when the response is received
    /// without blocking the current endpoint
    ///
    /// Includes proper error handling and structured logging to prevent zombie tasks
    #[tracing::instrument(skip(self, callback), fields(uuid = %uuid, participants_count = participants.len(), leader_idx = leader_idx))]
    pub async fn create_agregated_key_with_callback<F, Fut>(
        &self,
        uuid: Uuid,
        participants: Vec<BitVMXP2PAddress>,
        participants_keys: Option<Vec<PublicKey>>,
        leader_idx: u16,
        callback: F,
    ) -> Result<(), anyhow::Error>
    where
        F: FnOnce(Result<PublicKey, anyhow::Error>) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        debug!(
            "Creating aggregated key with callback for UUID: {}, participants: {}, leader_idx: {}",
            uuid,
            participants.len(),
            leader_idx
        );

        let message =
            IncomingBitVMXApiMessages::SetupKey(uuid, participants, participants_keys, leader_idx);

        self.rpc_client
            .send_request_with_callback(message, move |response| async move {
                // Parse and validate the response at the service level
                let result = match response {
                    Ok(OutgoingBitVMXApiMessages::AggregatedPubkey(_uuid, aggregated_pubkey)) => {
                        debug!("Successfully obtained aggregated key for UUID {}: {}", uuid, aggregated_pubkey);
                        Ok(aggregated_pubkey)
                    }
                    Ok(response) => {
                        error!(
                            "Unexpected response type for aggregated key creation UUID {}: {:?}",
                            uuid, response
                        );
                        Err(anyhow::anyhow!(
                            "Expected AggregatedPubkey response, got: {:?}",
                            response
                        ))
                    }
                    Err(e) => {
                        error!(
                            "RPC error during aggregated key creation for UUID {}: {:?}",
                            uuid, e
                        );
                        Err(e)
                    }
                };

                // Execute the callback with the parsed result
                callback(result).await;
            })
            .await
            .map_err(|e| {
                error!(
                    "Failed to send aggregated key creation request with callback for UUID {}: {:?}",
                    uuid, e
                );
                e
            })?;

        debug!(
            "Aggregated key creation request with callback submitted for UUID: {}",
            uuid
        );
        Ok(())
    }

    /// Send any BitVMX message with a callback - generic non-blocking version
    /// This method sends the request and executes the callback when the response is received
    /// without blocking the current endpoint
    ///
    /// Includes proper error handling and structured logging to prevent zombie tasks
    #[tracing::instrument(skip(self, callback), fields(message_type = %std::any::type_name::<IncomingBitVMXApiMessages>()))]
    pub async fn send_message_with_callback<F, Fut>(
        &self,
        message: IncomingBitVMXApiMessages,
        callback: F,
    ) -> Result<(), anyhow::Error>
    where
        F: FnOnce(Result<OutgoingBitVMXApiMessages, anyhow::Error>) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        trace!("Sending BitVMX message with callback");

        self.rpc_client
            .send_request_with_callback(message, move |response| async move {
                // Pass the raw response to the callback - let the callback handle parsing
                callback(response).await;
            })
            .await
            .map_err(|e| {
                error!("Failed to send BitVMX message with callback: {:?}", e);
                e
            })?;

        debug!("BitVMX message with callback submitted successfully");
        Ok(())
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
        let address = self.get_wallet_address()?;
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
        amount: u64,
    ) -> Result<PartialUtxo, anyhow::Error> {
        let response = self
            .rpc_client
            .send_request(IncomingBitVMXApiMessages::SendFunds(
                Uuid::new_v4(),
                destination.clone(),
                amount,
                None, // fee rate not needed for regtest
            ))
            .await?;

        if let OutgoingBitVMXApiMessages::FundsSent(_uuid, txid) = response {
            match destination {
                Destination::P2TR(x_only_pubkey, tap_leaves) => {
                    let pubkey = utils::bitcoin::xonly_to_pub_key(&x_only_pubkey)?;
                    let output_type = OutputType::taproot(amount, &pubkey, &tap_leaves)?;
                    Ok((txid, 0, Some(amount), Some(output_type)))
                }
                Destination::Address(address) => {
                    let script_pubkey = Address::from_str(&address)?
                        .assume_checked()
                        .script_pubkey();
                    let output_type = OutputType::ExternalUnknown { script_pubkey };
                    Ok((txid, 0, Some(amount), Some(output_type)))
                }
                Destination::P2WPKH(pubkey) => {
                    let output_type = OutputType::segwit_key(amount, &pubkey)?;
                    Ok((txid, 0, Some(amount), Some(output_type)))
                }
            }
        } else {
            Err(anyhow::anyhow!(
                "Expected Funds Sent response, got: {:?}",
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
    ) -> Result<(), anyhow::Error> {
        self.rpc_client
            .send_fire_and_forget(IncomingBitVMXApiMessages::Setup(
                program_id,
                program_type.to_string(),
                participants,
                leader_idx,
            ))
            .await?;

        Ok(())
    }

    pub fn protocol_cost(&self) -> u64 {
        bitvmx_client::program::protocols::dispute::protocol_cost()
    }

    // ----- Start internal methods -----

    /// Update P2P address
    async fn set_wallet_address(&mut self) -> Result<(), anyhow::Error> {
        let response = self
            .rpc_client
            .send_request(IncomingBitVMXApiMessages::GetFundingAddress(Uuid::new_v4()))
            .await?;

        if let OutgoingBitVMXApiMessages::FundingAddress(_uuid, address) = response {
            self.wallet_address = Some(address.assume_checked());
        } else {
            return Err(anyhow::anyhow!(
                "Expected Funding Address response, got: {:?}",
                response
            ));
        }
        let address = Address::from_str(&self.wallet_address.as_ref().unwrap().to_string())
            .unwrap()
            .assume_checked();
        let bitcoin_config = self.bitcoin_config.clone();

        // corre una rutina bloqueante sin trabar el runtime
        tokio::task::spawn_blocking(move || {
            let bitcoin_client = BitcoinClient::new(
                &bitcoin_config.url,
                &bitcoin_config.username,
                &bitcoin_config.password,
            )
            .unwrap();
            // each block gives a 50 BTC reward
            bitcoin_client.mine_blocks_to_address(1, &address).unwrap();
            bitcoin_client.mine_blocks(100).unwrap();
        })
        .await?;

        trace!("Updated wallet address in store");
        Ok(())
    }

    /// Update P2P address
    async fn set_p2p_address(&mut self) -> Result<(), anyhow::Error> {
        // Set P2P address
        let response = self
            .rpc_client
            .send_request(IncomingBitVMXApiMessages::GetCommInfo())
            .await?;
        if let OutgoingBitVMXApiMessages::CommInfo(comm_info) = response {
            self.p2p_address = Some(P2PAddress {
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
    async fn set_pub_key(&mut self) -> Result<(), anyhow::Error> {
        debug!("Create operator key from BitVMX");
        let pub_key_id = Uuid::new_v4();
        let response = self
            .rpc_client
            .send_request(IncomingBitVMXApiMessages::GetPubKey(pub_key_id, true))
            .await?;

        if let OutgoingBitVMXApiMessages::PubKey(_uuid, pub_key) = response {
            self.pub_key = Some(pub_key.to_string());
        } else {
            return Err(anyhow::anyhow!(
                "Expected Operator PubKey response, got: {:?}",
                response
            ));
        }
        trace!("Updated pub key in store");
        Ok(())
    }

    /// Update funding key
    async fn set_funding_key(&mut self) -> Result<(), anyhow::Error> {
        debug!("Create funding key for speedups from BitVMX");
        let speedup_key_id = Uuid::new_v4();
        let response = self
            .rpc_client
            .send_request(IncomingBitVMXApiMessages::GetPubKey(speedup_key_id, true))
            .await?;

        if let OutgoingBitVMXApiMessages::PubKey(_uuid, funding_key) = response {
            self.funding_key = Some(funding_key.to_string());
        } else {
            return Err(anyhow::anyhow!(
                "Expected Funding PubKey response, got: {:?}",
                response
            ));
        }
        trace!("Updated funding key in store");
        Ok(())
    }

    /// Setup BitVMX
    pub async fn initial_setup(&mut self) -> Result<(), anyhow::Error> {
        debug!("Get BitVMX info and initial keys setup");

        self.set_p2p_address().await?;

        // If keys do not exist, setup keys
        if self.get_pub_key().is_none() {
            debug!("No keys found, creating them");
            // Set operator pub key
            self.set_pub_key().await?;

            // Set funding key
            self.set_funding_key().await?;
        }

        self.set_wallet_address().await?;

        Ok(())
    }

    // ----- End internal methods -----
}
