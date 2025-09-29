use crate::rpc::chained_map::ChainedMap;
use crate::rpc::correlation::{request_to_correlation_id, response_to_correlation_id};
use bitvmx_broker::identification::allow_list::AllowList;
use bitvmx_broker::identification::identifier::Identifier;
use bitvmx_broker::rpc::client::Client;
use bitvmx_broker::rpc::tls_helper::Cert;
use bitvmx_broker::rpc::BrokerConfig;
use bitvmx_client::types::{IncomingBitVMXApiMessages, OutgoingBitVMXApiMessages};
use std::net::IpAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::{oneshot, Mutex};
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tracing::{debug, info, trace, warn, Instrument};

const REQUEST_TIMEOUT: u64 = 240; // 240 seconds = 4 minutes
const SLEEP_INTERVAL: u64 = 10; // 10 milliseconds
const CHECK_SHUTDOWN_INTERVAL: u64 = 100; // 100 milliseconds

/// BitVMX RPC Client with direct message sending
#[derive(Debug, Clone)]
pub struct RpcClient {
    /// Internal Broker RPC client
    client: Client,
    /// My ID for sending messages
    my_id: u8,
    /// Target identifier for sending messages
    to_identifier: Identifier,
    /// Pending responses waiting to be matched with correlation IDs
    pending_responses: Arc<Mutex<ChainedMap<String, oneshot::Sender<OutgoingBitVMXApiMessages>>>>,
    /// Ready flag
    ready: Arc<AtomicBool>,
}

impl RpcClient {
    /// Start a new RPC service
    /// Initialize the Broker RPC client with the specified port
    pub fn connect(
        broker_port: u16,
        broker_ip: Option<IpAddr>,
        shutdown_tx: &Sender<()>,
    ) -> Result<(Arc<Self>, JoinHandle<Result<(), anyhow::Error>>), anyhow::Error> {
        let bitvmx_key_file = "config/keys/bitvmx.key";
        let bitvmx_cert = Cert::from_key_file(bitvmx_key_file).map_err(|e| {
            anyhow::anyhow!("Failed to create certificate from file {bitvmx_key_file} err: {e:?}")
        })?;
        let bitvmx_identifier = Identifier::new(bitvmx_cert.get_pubk_hash()?, 0);
        debug!("BitVMX identifier: {:?}", bitvmx_identifier);

        let l2_key_file = "config/keys/l2.key";
        let l2_cert = Cert::from_key_file(l2_key_file).map_err(|e| {
            anyhow::anyhow!("Failed to create certificate from file {l2_key_file} err: {e:?}")
        })?;
        let l2_identifier = Identifier::new(l2_cert.get_pubk_hash()?, 0);
        debug!("L2 identifier: {:?}", l2_identifier);

        // Create allow list
        let allow_list = AllowList::new();
        allow_list.lock().unwrap().allow_all();

        // Create broker client
        let config = BrokerConfig::new(broker_port, broker_ip, bitvmx_cert.get_pubk_hash()?);
        let client = Client::new(&config, l2_cert.clone(), allow_list);

        // Create RPC client
        let rpc_client = Arc::new(Self {
            client,
            my_id: 0,
            to_identifier: bitvmx_identifier.clone(),
            pending_responses: Arc::new(Mutex::new(ChainedMap::new())),
            ready: Arc::new(AtomicBool::new(false)),
        });

        let listener_task =
            Self::spawn_listener(rpc_client.clone(), l2_identifier.clone(), shutdown_tx);

        Ok((rpc_client, listener_task))
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
            let mut pending_responses = self.pending_responses.lock().await;
            pending_responses.insert(correlation_id.to_string(), tx);
        }
        Ok(rx)
    }

    async fn get_response(
        &self,
        correlation_id: &str,
        rx: oneshot::Receiver<OutgoingBitVMXApiMessages>,
    ) -> Result<OutgoingBitVMXApiMessages, anyhow::Error> {
        // Wait for response with timeout
        let response = tokio::time::timeout(Duration::from_secs(REQUEST_TIMEOUT), rx)
            .await
            .map_err(|_| {
                anyhow::anyhow!(
                    "Request timed out after {} seconds for correlation_id: {}",
                    REQUEST_TIMEOUT,
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
            "Received from BitVMX response: {:?} message: {}",
            correlation_id,
            self.format_message(&response)
        );
        Ok(response)
    }

    pub async fn send_request(
        &self,
        message: IncomingBitVMXApiMessages,
    ) -> Result<OutgoingBitVMXApiMessages, anyhow::Error> {
        let correlation_id = request_to_correlation_id(&message)?;
        debug!(
            "Sending to BitVMX and waiting for response, request correlation_id: {:?} message: {:?}",
            correlation_id, message
        );
        let rx = self.add_response_handler(&correlation_id).await?;

        self.send_message(message).await?;

        let response = self.get_response(&correlation_id, rx).await?;
        debug!(
            "Received from BitVMX response: {:?} message: {}",
            correlation_id,
            self.format_message(&response)
        );
        Ok(response)
    }

    fn format_message(&self, message: &OutgoingBitVMXApiMessages) -> String {
        match message {
            OutgoingBitVMXApiMessages::Transaction(uuid, transaction_status, name) => {
                format!(
                    "Transaction message: uuid: {:?}, tx_id: {:?}, name: {:?}",
                    uuid, transaction_status.tx_id, name
                )
            }
            OutgoingBitVMXApiMessages::TransactionInfo(uuid, name, transaction) => {
                format!(
                    "TransactionInfo message: uuid: {:?}, name: {:?}, tx_id: {:?}",
                    uuid,
                    name,
                    transaction.compute_txid()
                )
            }
            _ => format!("message: {:?}", message),
        }
    }

    async fn send_message(&self, message: IncomingBitVMXApiMessages) -> Result<(), anyhow::Error> {
        // Serialize the message
        let serialized_msg = serde_json::to_string(&message)?;

        trace!("Sending message to BitVMX: {:?}", serialized_msg);

        // Send the message directly to BitVMX
        self.client
            .async_send_msg(self.my_id, self.to_identifier.clone(), serialized_msg)
            .await
            .map_err(|e| anyhow::anyhow!("Send message to BitVMX failed: {e}"))?;

        trace!("Sent message to BitVMX: {:?}", message);
        Ok(())
    }

    pub async fn send_fire_and_forget(
        &self,
        message: IncomingBitVMXApiMessages,
    ) -> Result<String, anyhow::Error> {
        let correlation_id = request_to_correlation_id(&message)?;
        debug!(
            "Sending fire-and-forget to BitVMX request: {:?} message: {:?}",
            correlation_id, message
        );
        self.send_message(message).await?;

        Ok(correlation_id)
    }

    async fn handle_response(&self, resp: String) -> Result<(), anyhow::Error> {
        // Deserialize the response
        let response = serde_json::from_str(&resp)?;

        let correlation_id = response_to_correlation_id(&response)?;
        trace!(
            "Received response: {:?} message: {:?}",
            correlation_id,
            response
        );

        let waiting_for_response = {
            let mut queue = self.pending_responses.lock().await;
            queue.drain_all_for_key(&correlation_id)?
        };

        if waiting_for_response.is_empty() {
            info!(
                "No response handler for correlation ID: {}, type: {:?}",
                correlation_id, response
            );
            return Ok(());
        }

        // Send the response to all pending handlers for this correlation ID
        for tx in waiting_for_response {
            if let Err(e) = tx.send(response.clone()) {
                warn!(
                    "Failed to send response to handler for correlation ID {}: {:?}",
                    correlation_id, e
                );
            }
        }

        Ok(())
    }

    fn spawn_listener(
        service: Arc<RpcClient>,
        my_identifier: Identifier,
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
                            Duration::from_millis(CHECK_SHUTDOWN_INTERVAL),
                            service.client.async_get_msg(my_identifier.clone())
                        ) => {
                            match result {
                                Ok(msg_result) => {
                                    match msg_result {
                                        Ok(Some(msg)) => {
                                            trace!("Received message from BitVMX: {:?}", msg);
                                            service.handle_response(msg.msg).await?;
                                            service.client.async_ack(my_identifier.clone(), msg.uid).await?;
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
                _ = sleep(Duration::from_millis(SLEEP_INTERVAL)) => {
                    if self.is_ready() {
                        break;
                    }
                }
            }
        }
    }
}
