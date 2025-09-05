use crate::models::{AggregatedKey, P2PAddress, WalletBalance};
use crate::rpc::rpc_client::RpcClient;
use crate::config::BitcoinConfig;
use bitcoin::{Address, PublicKey};
use bitvmx_client::types::{IncomingBitVMXApiMessages, OutgoingBitVMXApiMessages};
use std::str::FromStr;
use std::sync::Arc;
use tracing::{debug, trace};
use uuid::Uuid;
use bitvmx_bitcoin_rpc::bitcoin_client::BitcoinClient;
use bitvmx_bitcoin_rpc::bitcoin_client::BitcoinClientApi;

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
        self.wallet_address.as_ref().ok_or(anyhow::anyhow!("Wallet address not found"))?;
        Ok(self.wallet_address.as_ref().unwrap())
    }

    /// Create aggregated key
    pub async fn create_agregated_key(
        &self,
        uuid: Uuid,
        p2p_addresses: Vec<P2PAddress>,
        operator_keys: Option<Vec<PublicKey>>,
        leader_idx: u16,
    ) -> Result<AggregatedKey, anyhow::Error> {
        trace!("Create aggregated key from BitVMX");
        let addresses = p2p_addresses
            .iter()
            .map(|p2p| bitvmx_client::types::P2PAddress {
                address: p2p.address.clone(),
                peer_id: bitvmx_client::types::PeerId(p2p.peer_id.clone()),
            })
            .collect();
        let message =
            IncomingBitVMXApiMessages::SetupKey(uuid, addresses, operator_keys, leader_idx);

        let response = self.rpc_client.send_request(message).await?;

        if let OutgoingBitVMXApiMessages::AggregatedPubkey(uuid, aggregated_pubkey) = response {
            trace!("Obtained aggregated key: {:?}", aggregated_pubkey);
            Ok(AggregatedKey {
                uuid: uuid.to_string(),
                aggregated_key: aggregated_pubkey.to_string(),
            })
        } else {
            Err(anyhow::anyhow!(
                "Expected AggregatedPubkey response, got: {:?}",
                response
            ))
        }
    }

    /// Get aggregated key
    pub async fn aggregated_key(&self, uuid: Uuid) -> Result<AggregatedKey, anyhow::Error> {
        trace!("Get aggregated key from BitVMX");
        let response = self
            .rpc_client
            .send_request(IncomingBitVMXApiMessages::GetAggregatedPubkey(uuid))
            .await?;
        if let OutgoingBitVMXApiMessages::AggregatedPubkey(uuid, aggregated_pubkey) = response {
            trace!("Obtained aggregated key: {:?}", aggregated_pubkey);
            Ok(AggregatedKey {
                uuid: uuid.to_string(),
                aggregated_key: aggregated_pubkey.to_string(),
            })
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
        let balance_response = self
            .rpc_client
            .send_request(IncomingBitVMXApiMessages::GetFundingBalance(Uuid::new_v4()))
            .await?;

        if let OutgoingBitVMXApiMessages::FundingBalance(_uuid, balance) = balance_response {
            Ok(WalletBalance {
                address: address.to_string(),
                balance: balance
            })
        } else {
            Err(anyhow::anyhow!(
                "Expected Funding Address response, got: {:?}",
                balance_response
            ))
        }
    }

    /// Update P2P address
    async fn set_wallet_address(&mut self) -> Result<(), anyhow::Error> {
        // TODO use Wallet from bitvmx once bdk-wallet is merged
        let address_response = self
            .rpc_client
            .send_request(IncomingBitVMXApiMessages::GetFundingAddress(Uuid::new_v4()))
            .await?;

        if let OutgoingBitVMXApiMessages::FundingAddress(_uuid, address) = address_response {
            self.wallet_address = Some(address.assume_checked());
        } else {
            return Err(anyhow::anyhow!(
                "Expected Funding Address response, got: {:?}",
                address_response
            ));
        }
        let address = Address::from_str(&self.wallet_address.as_ref().unwrap().to_string()).unwrap().assume_checked();
        let bitcoin_config = self.bitcoin_config.clone();

        // corre una rutina bloqueante sin trabar el runtime
        let _ = tokio::task::spawn_blocking(move || {
            let bitcoin_client = BitcoinClient::new(&bitcoin_config.url, &bitcoin_config.username, &bitcoin_config.password).unwrap();
            // each block gives a 50 BTC reward
            bitcoin_client.mine_blocks_to_address(1, &address).unwrap();
            bitcoin_client.mine_blocks(100).unwrap();
        }).await?;

        

        trace!("Updated wallet address in store");
        Ok(())
    }

    /// Update P2P address
    async fn set_p2p_address(&mut self) -> Result<(), anyhow::Error> {
        // Set P2P address
        let comm_info_response = self
            .rpc_client
            .send_request(IncomingBitVMXApiMessages::GetCommInfo())
            .await?;
        if let OutgoingBitVMXApiMessages::CommInfo(comm_info) = comm_info_response {
            self.p2p_address = Some(P2PAddress {
                address: comm_info.address.clone(),
                peer_id: comm_info.peer_id.to_string(),
            });
        } else {
            return Err(anyhow::anyhow!(
                "Expected Comm Info response, got: {:?}",
                comm_info_response
            ));
        }
        trace!("Updated P2P address in store");
        Ok(())
    }

    /// Update pub key
    async fn set_pub_key(&mut self) -> Result<(), anyhow::Error> {
        debug!("Create operator key from BitVMX");
        let pub_key_id = Uuid::new_v4();
        let pub_key_response = self
            .rpc_client
            .send_request(IncomingBitVMXApiMessages::GetPubKey(pub_key_id, true))
            .await?;

        if let OutgoingBitVMXApiMessages::PubKey(_uuid, pub_key) = pub_key_response {
            self.pub_key = Some(pub_key.to_string());
        } else {
            return Err(anyhow::anyhow!(
                "Expected Operator PubKey response, got: {:?}",
                pub_key_response
            ));
        }
        trace!("Updated pub key in store");
        Ok(())
    }

    /// Update funding key
    async fn set_funding_key(&mut self) -> Result<(), anyhow::Error> {
        debug!("Create funding key for speedups from BitVMX");
        let speedup_key_id = Uuid::new_v4();
        let funding_key_response = self
            .rpc_client
            .send_request(IncomingBitVMXApiMessages::GetPubKey(speedup_key_id, true))
            .await?;

        if let OutgoingBitVMXApiMessages::PubKey(_uuid, funding_key) = funding_key_response {
            self.funding_key = Some(funding_key.to_string());
        } else {
            return Err(anyhow::anyhow!(
                "Expected Funding PubKey response, got: {:?}",
                funding_key_response
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

}
