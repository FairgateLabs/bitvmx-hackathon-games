use std::thread::sleep;
use std::time::Duration;

use crate::config::BitcoinConfig;
use bitvmx_bitcoin_rpc::bitcoin_client::BitcoinClient;
use bitvmx_bitcoin_rpc::bitcoin_client::BitcoinClientApi;
use bitvmx_client::bitcoin::Address;
use tracing::instrument;
use tracing::trace;

#[derive(Debug, Clone)]
pub struct BitcoinService {
    pub bitcoin_config: BitcoinConfig,
}

impl BitcoinService {
    pub fn new(bitcoin_config: BitcoinConfig) -> Self {
        Self {
            bitcoin_config: bitcoin_config.clone(),
        }
    }

    /// Mine blocks
    #[instrument(skip(self))]
    pub async fn mine_blocks(&self, blocks: u64) -> Result<(), anyhow::Error> {
        trace!("Mining {blocks} blocks");
        let bitcoin_config = self.bitcoin_config.clone();
        // run a blocking routine without blocking the runtime
        tokio::task::spawn_blocking(move || {
            // create the bitcoin client
            let bitcoin_client = BitcoinClient::new(
                &bitcoin_config.url,
                &bitcoin_config.username,
                &bitcoin_config.password,
            )
            .unwrap();

            // mine blocks
            bitcoin_client.mine_blocks(blocks).unwrap();
        })
        .await
        .map_err(|e| anyhow::anyhow!("Failed to mine blocks for funding wallet address: {e:?}"))?;

        trace!("Mined {blocks} blocks");
        Ok(())
    }

    /// Mine blocks to address then mine 100 blocks for maturity
    #[instrument(skip(self))]
    pub async fn mine_blocks_to_address(
        &self,
        blocks: u64,
        address: Address,
    ) -> Result<(), anyhow::Error> {
        trace!("Mining {blocks} blocks to address: {address}");
        let bitcoin_config = self.bitcoin_config.clone();
        // run a blocking routine without blocking the runtime
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
                .mine_blocks_to_address(blocks, &address)
                .unwrap();

            // mine 100 blocks for maturity
            // we split it to make it easier for the client to process
            bitcoin_client.mine_blocks(50).unwrap();
            sleep(Duration::from_secs(3));
            bitcoin_client.mine_blocks(50).unwrap();
            sleep(Duration::from_secs(3));
        })
        .await
        .map_err(|e| anyhow::anyhow!("Failed to mine blocks for funding wallet address: {e:?}"))?;

        trace!("Mined {blocks} blocks coinbase and 100 blocks maturity");
        Ok(())
    }
}
