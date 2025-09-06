use std::net::IpAddr;
use crate::rpc::chained_map::ChainedMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use bitvmx_broker::rpc::{BrokerConfig, async_client::AsyncClient};
use bitvmx_client::{
    types::{IncomingBitVMXApiMessages, OutgoingBitVMXApiMessages},
};
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::task::JoinHandle;
use tracing::{debug, info, trace, warn, Instrument};

use tokio::sync::{mpsc, oneshot, Mutex};
use tokio::time::sleep;
use std::time::Duration;



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
    pub fn connect( my_id:u32, to_id:u32, broker_port: u16, broker_ip: Option<IpAddr>, shutdown_tx: &Sender<()>) -> (Arc<Self>,JoinHandle<Result<(), anyhow::Error>>, JoinHandle<Result<(), anyhow::Error>>) {
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

    pub async fn send_request(&self, message: IncomingBitVMXApiMessages) -> Result<OutgoingBitVMXApiMessages, anyhow::Error> {
        let correlation_id = self.request_to_correlation_id(&message)?;
        debug!("Sending to BitVMX request: {:?} message: {:?}", correlation_id, message);
        let (tx, rx) = oneshot::channel();
        {
            let mut pending = self.pending.lock().await;
            pending.insert(correlation_id.clone(), tx);
        }

        self.outgoing.send((correlation_id.clone(), message)).await?;
        // TODO add timeout
        let response = rx.await?;
        debug!("Received from BitVMX response: {:?} message: {:?}", correlation_id, response);
        Ok(response)
    }

    pub async fn send_fire_and_forget(&self, message: IncomingBitVMXApiMessages) -> Result<(), anyhow::Error> {
        let correlation_id = self.request_to_correlation_id(&message)?;
        debug!("Sending fire-and-forget to BitVMX request: {:?} message: {:?}", correlation_id, message);

        self.outgoing.send((correlation_id.clone(), message)).await?;
        Ok(())
    }

    async fn handle_response(&self, resp: String) -> Result<(), anyhow::Error> {
        // Deserialize the response
        let response = serde_json::from_str(&resp)?;

        let correlation_id = self.response_to_correlation_id(&response)?;
        trace!("Received response: {:?} message: {:?}", correlation_id, response);

        let tx = {
            let mut pending = self.pending.lock().await;
            let optional_tx = pending.remove_first_for_key(&correlation_id)?;
            if optional_tx.is_none() {
                warn!("No response handler found for correlation ID: {}", correlation_id);
                return Ok(());
            }
            let tx = optional_tx.unwrap();
            tx
        };

        tx.send(response).map_err(|e| anyhow::anyhow!("failed to send response: {:?}", e))?;

        Ok(())
    }

    fn spawn_sender(service: Arc<Self>, mut rx: mpsc::Receiver<(String, IncomingBitVMXApiMessages)>, my_id:u32, to_id:u32, shutdown_tx: &Sender<()>) -> JoinHandle<Result<(), anyhow::Error>> {
        let mut shutdown_rx = shutdown_tx.subscribe();
        tokio::spawn(
            async move {
                info!("Start rpc sender");
                while let Some((_id, msg)) = rx.recv().await {
                    // Serialize the message
                    let serialized_msg = serde_json::to_string(&msg)?;
                    // Send the message to BitVMX
                    match service.client.send_msg(my_id, to_id, serialized_msg).await {
                        Ok(resp) => trace!("Sent message to BitVMX: {:?} result: {:?}", msg, resp),
                        Err(e) => {
                            return Err(anyhow::anyhow!("Send message to BitVMX failed: {e}"));
                        },
                    }
                    if shutdown_rx.try_recv().is_ok() {
                        trace!("Shutting down...");
                        break;
                    }
                    sleep(std::time::Duration::from_millis(10)).await;
                }
                info!("Channel closed, exiting loop");
                Ok::<_, anyhow::Error>(()) // coercion to Result
            }
            .instrument(tracing::info_span!("rpc_sender"))
        )
    }

    fn spawn_listener(service: Arc<RpcClient>, my_id:u32, shutdown_tx: &Sender<()>) -> JoinHandle<Result<(), anyhow::Error>> {
        let mut shutdown_rx = shutdown_tx.subscribe();
        tokio::spawn(
            async move {
                info!("Start rpc listener");
                let mut first_time = true;
                loop {
                    if shutdown_rx.try_recv().is_ok() {
                        trace!("Shutting down...");
                        break;
                    }

                    match service.client.get_msg(my_id).await {
                        Ok(Some(msg)) => {
                            trace!("Received message from BitVMX: {:?}", msg);
                            service.handle_response( msg.msg).await?;
                            service.client.ack(my_id, msg.uid).await?;
                        },
                        Ok(None) => { 
                            // No message received, sleep and continue loop
                        },
                        Err(e) => {
                            return Err(anyhow::anyhow!("Get message from BitVMX failed: {e}"));
                        },
                    }
                    
                    if first_time {
                        first_time = false;
                        service.set_ready();
                    }
                    sleep(std::time::Duration::from_millis(10)).await;
                }
                Ok::<_, anyhow::Error>(()) // coercion to Result
            }
            .instrument(tracing::info_span!("rpc_listener"))
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
            if shutdown_rx.try_recv().is_ok() {
                trace!("Exiting wait for ready loop...");
                break;
            }
            if self.is_ready() {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }

    /// Convert the message to send to BitVMX to a correlation ID
    fn request_to_correlation_id(&self, message: &IncomingBitVMXApiMessages) -> Result<String, anyhow::Error> {
        // Serialize the message
        match message {
            IncomingBitVMXApiMessages::SetVar(uuid, _key, _value) => {
                Ok(uuid.to_string())
            },
            IncomingBitVMXApiMessages::GetVar(uuid, _key) => {
                Ok(uuid.to_string())
            },
            IncomingBitVMXApiMessages::GetTransaction(uuid, _txid) => {
                Ok(uuid.to_string())
            },
            IncomingBitVMXApiMessages::SendFunds(uuid, _destination, _amount, _fee) => {
                Ok(uuid.to_string())
            },
            IncomingBitVMXApiMessages::GetFundingBalance(uuid) => {
                Ok(uuid.to_string())
            },
            IncomingBitVMXApiMessages::GetFundingAddress(uuid) => {
                Ok(uuid.to_string())
            },
            IncomingBitVMXApiMessages::SetupKey(uuid, _addresses, _operator_key, _funding_key) => {
                Ok(uuid.to_string())
            },
            IncomingBitVMXApiMessages::GetAggregatedPubkey(uuid) => {
                Ok(uuid.to_string())
            },
            IncomingBitVMXApiMessages::GetPubKey(uuid, _new_key) => {
                Ok(uuid.to_string())
            },
            IncomingBitVMXApiMessages::GetCommInfo() => {
                Ok("get_comm_info".to_string())
            },
            IncomingBitVMXApiMessages::Ping() => {
                Ok("ping".to_string())
            },
            _ => {
                Err(anyhow::anyhow!("unhandled request message type: {:?}", message))
            }
        }
    }

    /// Convert the response received from BitVMX to a correlation ID
    fn response_to_correlation_id(&self, response: &OutgoingBitVMXApiMessages) -> Result<String, anyhow::Error> {
        match response {
            OutgoingBitVMXApiMessages::Variable(uuid, _key, _value) => {
                Ok(uuid.to_string())
            },
            OutgoingBitVMXApiMessages::Transaction(uuid, _transaction_status, _transaction) => {
                Ok(uuid.to_string())
            },
            OutgoingBitVMXApiMessages::FundsSent(uuid, _txid) => {
                Ok(uuid.to_string())
            },
            OutgoingBitVMXApiMessages::FundingBalance(uuid, _balance) => {
                Ok(uuid.to_string())
            },
            OutgoingBitVMXApiMessages::WalletNotReady(uuid) => {
                Ok(uuid.to_string())
            },
            OutgoingBitVMXApiMessages::FundingAddress(uuid, _address) => {
                Ok(uuid.to_string())
            },
            OutgoingBitVMXApiMessages::AggregatedPubkeyNotReady(uuid) => {
                Ok(uuid.to_string())
            },
            OutgoingBitVMXApiMessages::AggregatedPubkey(uuid, _aggregated_pubkey) => {
                Ok(uuid.to_string())
            },
            OutgoingBitVMXApiMessages::PubKey(uuid, _pub_key) => {
                Ok(uuid.to_string())
            }
            OutgoingBitVMXApiMessages::CommInfo(_p2p_address) => {
                Ok("get_comm_info".to_string())
            }
            OutgoingBitVMXApiMessages::Pong() => {
                Ok("ping".to_string())
            }
            _ => {
                Err(anyhow::anyhow!("unhandled response message type: {:?}", response))
            }
        }
    }

}