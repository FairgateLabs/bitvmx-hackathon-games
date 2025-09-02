use std::net::IpAddr;
use crate::rpc::ordered_bag::OrderedBag;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use bitvmx_broker::rpc::{BrokerConfig, async_client::AsyncClient};
use bitvmx_client::{
    types::{IncomingBitVMXApiMessages, OutgoingBitVMXApiMessages::{self}, L2_ID, BITVMX_ID},
};
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::task::JoinHandle;
use tracing::{debug, error, info, trace, warn};

use tokio::sync::{mpsc, oneshot, Mutex};
use tokio::time::sleep;
use std::time::Duration;



/// BitVMX RPC Client with async message queue
#[derive(Debug, Clone)]
pub struct RpcService {
    /// Internal Broker RPC client
    client: AsyncClient,
    /// Outgoing messages
    outgoing: mpsc::Sender<(String, IncomingBitVMXApiMessages)>,
    /// Pending responses
    pending: Arc<Mutex<OrderedBag<String, oneshot::Sender<OutgoingBitVMXApiMessages>>>>,
    /// Ready flag
    ready: Arc<AtomicBool>,
}


impl RpcService {
    /// Start a new RPC service
    /// Initialize the Broker RPC client with the specified port
    pub fn connect(broker_port: u16, broker_ip: Option<IpAddr>, shutdown_tx: &Sender<()>) -> (Arc<Self>,JoinHandle<Result<(), anyhow::Error>>, JoinHandle<Result<(), anyhow::Error>>) {
        let config = BrokerConfig::new(broker_port, broker_ip);
        let client = AsyncClient::new(&config);

        let (tx, rx) = mpsc::channel(100);

        let service = Arc::new(RpcService {
            client,
            outgoing: tx,
            pending: Arc::new(Mutex::new(OrderedBag::new())),
            ready: Arc::new(AtomicBool::new(false)),
        });

        let sender_task = RpcService::spawn_sender(service.clone(), rx, shutdown_tx.subscribe());
        let listener_task = RpcService::spawn_listener(service.clone(), shutdown_tx.subscribe());

        (service, sender_task, listener_task)
    }

    fn request_to_correlation_id(&self, message: &IncomingBitVMXApiMessages) -> Result<String, anyhow::Error> {
        // Serialize the message
        match message {
            IncomingBitVMXApiMessages::SetupKey(uuid, _addresses, _operator_key, _funding_key) => {
                Ok(uuid.to_string())
            },
            IncomingBitVMXApiMessages::GetPubKey(uuid, _new_key) => {
                Ok(uuid.to_string())
            },
            IncomingBitVMXApiMessages::GetCommInfo() => {
                Ok("get_comm_info".to_string())
            },
            _ => {
                Err(anyhow::anyhow!("Unhandled message type: {:?}", message))
            }
        }
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

    fn response_to_correlation_id(&self, response: &OutgoingBitVMXApiMessages) -> Result<String, anyhow::Error> {
        match response {
            OutgoingBitVMXApiMessages::PubKey(uuid, _pub_key) => {
                Ok(uuid.to_string())
            }
            OutgoingBitVMXApiMessages::CommInfo(_p2p_address) => {
                Ok("get_comm_info".to_string())
            }
            _ => {
                Err(anyhow::anyhow!("Unhandled message type: {:?}", response))
            }
        }
    }

    async fn handle_response(&self, resp: String) -> Result<(), anyhow::Error> {
        // Deserialize the response
        let response = serde_json::from_str(&resp)?;

        let correlation_id = self.response_to_correlation_id(&response)?;
        trace!("[rpc_listener] received response: {:?} message: {:?}", correlation_id, response);

        let tx = {
            let mut pending = self.pending.lock().await;
            let optional_tx = pending.remove_first_for_key(&correlation_id)?;
            if optional_tx.is_none() {
                warn!("[rpc_listener] no response handler found for correlation ID: {}", correlation_id);
                return Ok(());
            }
            let tx = optional_tx.unwrap();
            tx
        };

        tx.send(response).map_err(|e| anyhow::anyhow!("[rpc_listener] failed to send response: {:?}", e))?;

        Ok(())
    }

    fn spawn_sender(service: Arc<Self>, mut rx: mpsc::Receiver<(String, IncomingBitVMXApiMessages)>, mut shutdown_rx: Receiver<()>) -> JoinHandle<Result<(), anyhow::Error>> {
        tokio::spawn(async move {
            info!("[rpc_sender] spawned");
            while let Some((_id, msg)) = rx.recv().await {
                // Serialize the message
                let serialized_msg = serde_json::to_string(&msg)?;
                // Send the message to BitVMX
                match service.client.send_msg(L2_ID, BITVMX_ID, serialized_msg).await {
                    Ok(resp) => trace!("[rpc_sender] sent message to BitVMX: {:?} result: {:?}", msg, resp),
                    Err(e) => error!("[rpc_sender] send message to BitVMX failed: {e}"),
                }
                if shutdown_rx.try_recv().is_ok() {
                    info!("[rpc_sender] shutting down...");
                    break;
                }
                sleep(std::time::Duration::from_millis(10)).await;
            }
            info!("[rpc_sender] channel closed, exiting loop");
            Ok::<_, anyhow::Error>(()) // coercion to Result
        })
    }

    fn spawn_listener(service: Arc<RpcService>, mut shutdown_rx: Receiver<()>) -> JoinHandle<Result<(), anyhow::Error>> {
        tokio::spawn(async move {
            info!("[rpc_listener] spawned");
            let mut first_time = true;
            loop {
                if shutdown_rx.try_recv().is_ok() {
                    info!("[rpc_listener] shutting down...");
                    break;
                }

                match service.client.get_msg(L2_ID).await {
                    Ok(Some(msg)) => {
                        trace!("[rpc_listener] received message from BitVMX: {:?}", msg);
                        service.handle_response( msg.msg).await?;
                        service.client.ack(L2_ID, msg.uid).await?;
                    },
                    Ok(None) => { 
                        // No message received, sleep and continue loop
                    },
                    Err(e) => {
                        error!("[rpc_listener] get message from BitVMX failed: {e}");
                        break;
                    },
                }
                
                if first_time {
                    first_time = false;
                    service.set_ready();
                }
                sleep(std::time::Duration::from_millis(10)).await;
            }
            Ok::<_, anyhow::Error>(()) // coercion to Result
        })
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
    pub async fn wait_for_ready(&self) {
        loop {
            if self.is_ready() {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }

}