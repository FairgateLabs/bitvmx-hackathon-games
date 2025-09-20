use crate::rpc::chained_map::ChainedMap;
use bitvmx_broker::rpc::async_client::AsyncClient;
use bitvmx_broker::rpc::BrokerConfig;
use bitvmx_client::types::{IncomingBitVMXApiMessages, OutgoingBitVMXApiMessages};
use std::net::IpAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::task::JoinHandle;
use tracing::{debug, info, trace, warn, Instrument};
use uuid::Uuid;

use std::time::Duration;
use tokio::sync::{mpsc, oneshot, Mutex};
use tokio::time::sleep;

const TIMEOUT: u64 = 120; // 2 minutes
const SLEEP_TIME: u64 = 10; // 10 milliseconds

/// BitVMX RPC Client with async message queue
#[derive(Debug, Clone)]
pub struct RpcClient {
    /// Internal Broker RPC client
    client: AsyncClient,
    /// Outgoing messages
    outgoing: mpsc::Sender<(String, IncomingBitVMXApiMessages)>,
    /// Pending responses
    pending: Arc<Mutex<ChainedMap<String, oneshot::Sender<OutgoingBitVMXApiMessages>>>>,
    /// Ready flag
    ready: Arc<AtomicBool>,
}

impl RpcClient {
    /// Start a new RPC service
    /// Initialize the Broker RPC client with the specified port
    pub fn connect(
        my_id: u32,
        to_id: u32,
        broker_port: u16,
        broker_ip: Option<IpAddr>,
        shutdown_tx: &Sender<()>,
    ) -> (
        Arc<Self>,
        JoinHandle<Result<(), anyhow::Error>>,
        JoinHandle<Result<(), anyhow::Error>>,
    ) {
        let config = BrokerConfig::new(broker_port, broker_ip);
        let client = AsyncClient::new(&config);

        let (tx, rx) = mpsc::channel(100);

        let service = Arc::new(RpcClient {
            client,
            outgoing: tx,
            pending: Arc::new(Mutex::new(ChainedMap::new())),
            ready: Arc::new(AtomicBool::new(false)),
        });

        let sender_task = RpcClient::spawn_sender(service.clone(), rx, my_id, to_id, shutdown_tx);
        let listener_task = RpcClient::spawn_listener(service.clone(), my_id, shutdown_tx);

        (service, sender_task, listener_task)
    }

    async fn add_response_handler(
        &self,
        correlation_id: &str,
    ) -> Result<oneshot::Receiver<OutgoingBitVMXApiMessages>, anyhow::Error> {
        trace!(
            "Adding response handler to queue for correlation id: {:?}",
            correlation_id
        );
        let (tx, rx) = oneshot::channel();
        {
            let mut pending = self.pending.lock().await;
            pending.insert(correlation_id.to_string(), tx);
        }
        Ok(rx)
    }

    async fn get_response(
        &self,
        correlation_id: &str,
        rx: oneshot::Receiver<OutgoingBitVMXApiMessages>,
    ) -> Result<OutgoingBitVMXApiMessages, anyhow::Error> {
        // Wait for response with timeout
        let response = tokio::time::timeout(Duration::from_secs(TIMEOUT), rx)
            .await
            .map_err(|_| {
                anyhow::anyhow!(
                    "Request timed out after {} seconds for correlation_id: {}",
                    TIMEOUT,
                    correlation_id
                )
            })?
            .map_err(|_| {
                anyhow::anyhow!(
                    "Channel closed while waiting for response for correlation_id: {}",
                    correlation_id
                )
            })?;

        Ok(response)
    }

    pub async fn wait_for_response(
        &self,
        correlation_id: String,
    ) -> Result<OutgoingBitVMXApiMessages, anyhow::Error> {
        debug!(
            "Waiting for BitVMX response for correlation id: {:?}",
            correlation_id
        );
        let rx = self.add_response_handler(&correlation_id).await?;
        let response = self.get_response(&correlation_id, rx).await?;
        debug!(
            "Received from BitVMX response: {:?} message: {:?}",
            correlation_id, response
        );
        Ok(response)
    }

    pub async fn send_request(
        &self,
        message: IncomingBitVMXApiMessages,
    ) -> Result<OutgoingBitVMXApiMessages, anyhow::Error> {
        let correlation_id = self.request_to_correlation_id(&message)?;
        debug!(
            "Sending to BitVMX and waiting for response, request correlation_id: {:?} message: {:?}",
            correlation_id, message
        );
        let rx = self.add_response_handler(&correlation_id).await?;

        self.send_message(&correlation_id, message).await?;

        let response = self.get_response(&correlation_id, rx).await?;
        debug!(
            "Received from BitVMX response: {:?} message: {:?}",
            correlation_id, response
        );
        Ok(response)
    }

    async fn send_message(
        &self,
        correlation_id: &str,
        message: IncomingBitVMXApiMessages,
    ) -> Result<(), anyhow::Error> {
        self.outgoing
            .send((correlation_id.to_string(), message))
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send message: {e}"))?;
        Ok(())
    }

    pub async fn send_fire_and_forget(
        &self,
        message: IncomingBitVMXApiMessages,
    ) -> Result<String, anyhow::Error> {
        let correlation_id = self.request_to_correlation_id(&message)?;
        debug!(
            "Sending fire-and-forget to BitVMX request: {:?} message: {:?}",
            correlation_id, message
        );
        self.send_message(&correlation_id, message).await?;

        Ok(correlation_id)
    }

    async fn handle_response(&self, resp: String) -> Result<(), anyhow::Error> {
        // Deserialize the response
        let response = serde_json::from_str(&resp)?;

        let correlation_id = self.response_to_correlation_id(&response)?;
        trace!(
            "Received response: {:?} message: {:?}",
            correlation_id,
            response
        );

        let pending_txs = {
            let mut pending = self.pending.lock().await;
            pending.drain_all_for_key(&correlation_id)?
        };

        if pending_txs.is_empty() {
            info!(
                "No response handler for correlation ID: {}, type: {:?}",
                correlation_id, response,
            );
            return Ok(());
        }

        // Send the response to all pending handlers for this correlation ID
        for tx in pending_txs {
            if let Err(e) = tx.send(response.clone()) {
                warn!(
                    "Failed to send response to handler for correlation ID {}: {:?}",
                    correlation_id, e
                );
            }
        }

        Ok(())
    }

    fn spawn_sender(
        service: Arc<Self>,
        mut rx: mpsc::Receiver<(String, IncomingBitVMXApiMessages)>,
        my_id: u32,
        to_id: u32,
        shutdown_tx: &Sender<()>,
    ) -> JoinHandle<Result<(), anyhow::Error>> {
        let mut shutdown_rx = shutdown_tx.subscribe();
        tokio::spawn(
            async move {
                info!("Start rpc sender");
                loop {
                    tokio::select! {
                        _ = shutdown_rx.recv() => {
                            warn!("Shutting down rpc sender...");
                            break;
                        }
                        msg_opt = rx.recv() => {
                            match msg_opt {
                                Some((_sender_id, msg)) => {
                                    // Serialize the message
                                    let serialized_msg = serde_json::to_string(&msg)?;
                                    // Send the message to BitVMX
                                    match service.client.send_msg(my_id, to_id, serialized_msg).await {
                                        Ok(resp) => trace!("Sent message to BitVMX: {:?} result: {:?}", msg, resp),
                                        Err(e) => {
                                            return Err(anyhow::anyhow!("Send message to BitVMX failed: {e}"));
                                        }
                                    }
                                    sleep(std::time::Duration::from_millis(SLEEP_TIME)).await;
                                }
                                None => {
                                    info!("Channel closed, exiting loop");
                                    break;
                                }
                            }
                        }
                    }
                }
                Ok::<_, anyhow::Error>(()) // coercion to Result
            }
            .instrument(tracing::info_span!("rpc_sender")),
        )
    }

    fn spawn_listener(
        service: Arc<RpcClient>,
        my_id: u32,
        shutdown_tx: &Sender<()>,
    ) -> JoinHandle<Result<(), anyhow::Error>> {
        let mut shutdown_rx = shutdown_tx.subscribe();
        tokio::spawn(
            async move {
                info!("Start rpc listener");
                let mut first_time = true;
                loop {
                    tokio::select! {
                        _ = shutdown_rx.recv() => {
                            warn!("Shutting down rpc listener...");
                            break;
                        }
                        result = tokio::time::timeout(
                            Duration::from_millis(SLEEP_TIME),
                            service.client.get_msg(my_id)
                        ) => {
                            match result {
                                Ok(msg_result) => {
                                    match msg_result {
                                        Ok(Some(msg)) => {
                                            trace!("Received message from BitVMX: {:?}", msg);
                                            service.handle_response(msg.msg).await?;
                                            service.client.ack(my_id, msg.uid).await?;
                                        }
                                        Ok(None) => {
                                            // No message received, continue loop
                                        }
                                        Err(e) => {
                                            return Err(anyhow::anyhow!(
                                                "Get message from BitVMX failed: {e}"
                                            ));
                                        }
                                    }

                                    if first_time {
                                        first_time = false;
                                        service.set_ready();
                                    }
                                }
                                Err(_timeout) => {
                                    // Timeout occurred, continue loop to check shutdown signal
                                }
                            }
                        }
                    }
                }
                Ok::<_, anyhow::Error>(()) // coercion to Result
            }
            .instrument(tracing::info_span!("rpc_listener")),
        )
    }

    /// Set the RPC client as ready
    fn set_ready(&self) {
        self.ready.store(true, Ordering::Release);
    }

    /// Check if the RPC client is ready
    pub fn is_ready(&self) -> bool {
        self.ready.load(Ordering::Acquire)
    }

    /// Wait for the RPC client to be ready
    pub async fn wait_for_ready(&self, mut shutdown_rx: Receiver<()>) {
        loop {
            tokio::select! {
                _ = shutdown_rx.recv() => {
                    trace!("Exiting wait for ready loop...");
                    break;
                }
                _ = tokio::time::sleep(Duration::from_millis(SLEEP_TIME)) => {
                    if self.is_ready() {
                        break;
                    }
                }
            }
        }
    }

    /// Convert the transaction name to a correlation ID
    pub fn tx_name_to_correlation_id(program_id: &Uuid, name: &str) -> String {
        format!("{program_id}_{name}")
    }

    /// Convert the message to send to BitVMX to a correlation ID
    fn request_to_correlation_id(
        &self,
        message: &IncomingBitVMXApiMessages,
    ) -> Result<String, anyhow::Error> {
        // Serialize the message
        match message {
            IncomingBitVMXApiMessages::Ping() => Ok("ping".to_string()),
            IncomingBitVMXApiMessages::SetVar(uuid, _key, _value) => Ok(uuid.to_string()),
            IncomingBitVMXApiMessages::SetWitness(uuid, _address, _witness) => Ok(uuid.to_string()),
            IncomingBitVMXApiMessages::SetFundingUtxo(utxo) => {
                Ok(format!("set_funding_utxo_{}_{}", utxo.txid, utxo.vout))
            }
            IncomingBitVMXApiMessages::GetVar(uuid, _key) => Ok(uuid.to_string()),
            IncomingBitVMXApiMessages::GetWitness(uuid, _address) => Ok(uuid.to_string()),
            IncomingBitVMXApiMessages::GetCommInfo() => Ok("get_comm_info".to_string()),
            IncomingBitVMXApiMessages::GetTransaction(uuid, _txid) => Ok(uuid.to_string()),
            IncomingBitVMXApiMessages::GetTransactionInfoByName(uuid, _name) => {
                Ok(uuid.to_string())
            }
            IncomingBitVMXApiMessages::GetHashedMessage(uuid, _name, _vout, _leaf) => {
                Ok(uuid.to_string())
            }
            IncomingBitVMXApiMessages::Setup(uuid, _program_type, _participants, _leader_idx) => {
                Ok(uuid.to_string())
            }
            IncomingBitVMXApiMessages::SubscribeToTransaction(uuid, _txid) => Ok(uuid.to_string()),
            IncomingBitVMXApiMessages::SubscribeUTXO() => Ok("subscribe_utxo".to_string()),
            IncomingBitVMXApiMessages::SubscribeToRskPegin() => {
                Ok("subscribe_rsk_pegin".to_string())
            }
            IncomingBitVMXApiMessages::GetSPVProof(_txid) => Ok(format!("spv_proof_{_txid}")),
            IncomingBitVMXApiMessages::DispatchTransaction(uuid, _transaction) => {
                Ok(uuid.to_string())
            }
            IncomingBitVMXApiMessages::DispatchTransactionName(uuid, name) => {
                Ok(Self::tx_name_to_correlation_id(uuid, name))
            }
            IncomingBitVMXApiMessages::SetupKey(uuid, _addresses, _operator_key, _funding_key) => {
                Ok(uuid.to_string())
            }
            IncomingBitVMXApiMessages::GetAggregatedPubkey(uuid) => Ok(uuid.to_string()),
            IncomingBitVMXApiMessages::GetProtocolVisualization(uuid) => {
                Ok(format!("protocol_visualization_{}", uuid))
            }
            IncomingBitVMXApiMessages::GetKeyPair(uuid) => Ok(uuid.to_string()),
            IncomingBitVMXApiMessages::GetPubKey(uuid, _new_key) => Ok(uuid.to_string()),
            IncomingBitVMXApiMessages::SignMessage(uuid, _payload_to_sign, _public_key_to_use) => {
                Ok(uuid.to_string())
            }
            IncomingBitVMXApiMessages::GenerateZKP(uuid, _payload_to_sign, _name) => {
                Ok(uuid.to_string())
            }
            IncomingBitVMXApiMessages::ProofReady(uuid) => Ok(uuid.to_string()),
            IncomingBitVMXApiMessages::GetZKPExecutionResult(uuid) => Ok(uuid.to_string()),
            IncomingBitVMXApiMessages::Encrypt(uuid, _payload_to_encrypt, _public_key_to_use) => {
                Ok(uuid.to_string())
            }
            IncomingBitVMXApiMessages::Decrypt(uuid, _payload_to_decrypt) => Ok(uuid.to_string()),
            IncomingBitVMXApiMessages::GetFundingBalance(uuid) => Ok(uuid.to_string()),
            IncomingBitVMXApiMessages::GetFundingAddress(uuid) => Ok(uuid.to_string()),
            IncomingBitVMXApiMessages::SendFunds(uuid, _destination, _fee) => Ok(uuid.to_string()),
            _ => Err(anyhow::anyhow!(
                "unhandled request message type: {:?}",
                message
            )),
        }
    }

    /// Convert the response received from BitVMX to a correlation ID
    fn response_to_correlation_id(
        &self,
        response: &OutgoingBitVMXApiMessages,
    ) -> Result<String, anyhow::Error> {
        match response {
            OutgoingBitVMXApiMessages::Pong() => Ok("ping".to_string()),
            OutgoingBitVMXApiMessages::Transaction(uuid, _transaction_status, name) => match name {
                Some(name) => Ok(Self::tx_name_to_correlation_id(uuid, name)),
                None => Ok(uuid.to_string()),
            },
            OutgoingBitVMXApiMessages::PeginTransactionFound(_txid, _transaction_status) => {
                Ok("rsk_pegin".to_string())
            }
            OutgoingBitVMXApiMessages::SpendingUTXOTransactionFound(
                uuid,
                _txid,
                _vout,
                _transaction_status,
            ) => Ok(uuid.to_string()),
            OutgoingBitVMXApiMessages::SetupCompleted(uuid) => Ok(uuid.to_string()),
            OutgoingBitVMXApiMessages::AggregatedPubkey(uuid, _aggregated_pubkey) => {
                Ok(uuid.to_string())
            }
            OutgoingBitVMXApiMessages::AggregatedPubkeyNotReady(uuid) => Ok(uuid.to_string()),
            OutgoingBitVMXApiMessages::ProtocolVisualization(uuid, _visualization) => {
                Ok(format!("protocol_visualization_{}", uuid))
            }
            OutgoingBitVMXApiMessages::TransactionInfo(uuid, _name, _transaction) => {
                Ok(uuid.to_string())
            }
            OutgoingBitVMXApiMessages::ZKPResult(uuid, _zkp_result, _zkp_proof) => {
                Ok(uuid.to_string())
            }
            OutgoingBitVMXApiMessages::CommInfo(_p2p_address) => Ok("get_comm_info".to_string()),
            OutgoingBitVMXApiMessages::KeyPair(uuid, _private_key, _public_key) => {
                Ok(uuid.to_string())
            }
            OutgoingBitVMXApiMessages::PubKey(uuid, _pub_key) => Ok(uuid.to_string()),
            OutgoingBitVMXApiMessages::SignedMessage(
                uuid,
                _signature_r,
                _signature_s,
                _recovery_id,
            ) => Ok(uuid.to_string()),
            OutgoingBitVMXApiMessages::Variable(uuid, _key, _value) => Ok(uuid.to_string()),
            OutgoingBitVMXApiMessages::Witness(uuid, _key, _witness) => Ok(uuid.to_string()),
            OutgoingBitVMXApiMessages::NotFound(uuid, _key) => Ok(uuid.to_string()),
            OutgoingBitVMXApiMessages::HashedMessage(uuid, _name, _vout, _leaf, _) => {
                Ok(uuid.to_string())
            }
            OutgoingBitVMXApiMessages::ProofReady(uuid) => Ok(uuid.to_string()),
            OutgoingBitVMXApiMessages::ProofNotReady(uuid) => Ok(uuid.to_string()),
            OutgoingBitVMXApiMessages::ProofGenerationError(uuid, _error) => Ok(uuid.to_string()),
            OutgoingBitVMXApiMessages::SPVProof(txid, _spv_proof) => {
                Ok(format!("spv_proof_{txid}"))
            }
            OutgoingBitVMXApiMessages::Encrypted(uuid, _encrypted_message) => Ok(uuid.to_string()),
            OutgoingBitVMXApiMessages::Decrypted(uuid, _decrypted_message) => Ok(uuid.to_string()),
            OutgoingBitVMXApiMessages::FundingAddress(uuid, _address) => Ok(uuid.to_string()),
            OutgoingBitVMXApiMessages::FundingBalance(uuid, _balance) => Ok(uuid.to_string()),
            OutgoingBitVMXApiMessages::FundsSent(uuid, _txid) => Ok(uuid.to_string()),
            OutgoingBitVMXApiMessages::WalletNotReady(uuid) => Ok(uuid.to_string()),
            OutgoingBitVMXApiMessages::WalletError(uuid, _error) => Ok(uuid.to_string()),
            _ => Err(anyhow::anyhow!(
                "unhandled response message type: {:?}",
                response
            )),
        }
    }
}
