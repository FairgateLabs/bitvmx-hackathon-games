use crate::models::P2PAddress;
use bitvmx_client::bitcoin::{Address, PublicKey};

use std::sync::Arc;
use tokio::sync::RwLock;
#[derive(Debug, Clone)]
pub struct BitvmxInfo {
    pub p2p_address: Option<P2PAddress>,
    pub pub_key: Option<String>,
    pub funding_key: Option<String>,
    pub wallet_address: Option<Address>,
}

#[derive(Debug, Clone)]
pub struct BitvmxStore {
    pub info: Arc<RwLock<BitvmxInfo>>,
}

impl Default for BitvmxStore {
    fn default() -> Self {
        Self::new()
    }
}

impl BitvmxStore {
    pub fn new() -> Self {
        Self {
            info: Arc::new(RwLock::new(BitvmxInfo {
                p2p_address: None,
                pub_key: None,
                funding_key: None,
                wallet_address: None,
            })),
        }
    }

    /// Get pub key
    pub async fn get_pub_key(&self) -> Result<Option<String>, anyhow::Error> {
        let bitvmx_info = self.info.read().await;
        Ok(bitvmx_info.pub_key.clone())
    }

    /// Get funding key
    pub async fn get_funding_key(&self) -> Result<Option<String>, anyhow::Error> {
        let bitvmx_info = self.info.read().await;
        Ok(bitvmx_info.funding_key.clone())
    }

    /// Get P2P address
    pub async fn get_p2p_address(&self) -> Result<Option<P2PAddress>, anyhow::Error> {
        let bitvmx_info = self.info.read().await;
        Ok(bitvmx_info.p2p_address.clone())
    }

    /// Get wallet address
    pub async fn get_wallet_address(&self) -> Result<Option<Address>, anyhow::Error> {
        let bitvmx_info = self.info.read().await;
        Ok(bitvmx_info.wallet_address.clone())
    }

    /// Update P2P address
    pub async fn set_wallet_address(&self, wallet_address: Address) -> Result<(), anyhow::Error> {
        self.info.write().await.wallet_address = Some(wallet_address.clone());
        Ok(())
    }

    /// Update P2P address
    pub async fn set_p2p_address(&self, p2p_address: P2PAddress) -> Result<(), anyhow::Error> {
        self.info.write().await.p2p_address = Some(p2p_address);
        Ok(())
    }

    /// Update pub key
    pub async fn set_pub_key(&self, pub_key: PublicKey) -> Result<(), anyhow::Error> {
        self.info.write().await.pub_key = Some(pub_key.to_string());
        Ok(())
    }

    /// Update funding key
    pub async fn set_funding_key(&self, funding_key: PublicKey) -> Result<(), anyhow::Error> {
        self.info.write().await.funding_key = Some(funding_key.to_string());
        Ok(())
    }
}
