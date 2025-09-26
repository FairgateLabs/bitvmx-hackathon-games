use crate::models::{
    AddNumbersGame, AddNumbersGameStatus, GameOutcome, GameReason, P2PAddress, PlayerRole, Utxo,
};
use crate::services::{BitvmxService, WorkerService};
use crate::stores::AddNumbersStore;
use bitvmx_client::bitcoin::PublicKey;
use bitvmx_client::bitcoin_coordinator::TransactionStatus;
use bitvmx_client::bitvmx_wallet::wallet::Destination;
use bitvmx_client::program::participant::CommsAddress as BitVMXP2PAddress;
use bitvmx_client::program::protocols::dispute;
use bitvmx_client::program::variables::VariableTypes;
use bitvmx_client::protocol_builder::types::OutputType;
use std::str::FromStr;
use std::sync::Arc;
use tokio::task::JoinSet;
use tracing::{debug, error, info, instrument};
use uuid::Uuid;

// File path should be the relative path from the bitvmx-client to the program definition file
const PROGRAM_PATH: &str = "./verifiers/add-test-with-const-pre.yaml";

#[derive(Debug)]
pub struct AddNumbersService {
    game_store: Arc<AddNumbersStore>,
    bitvmx_service: Arc<BitvmxService>,
}

impl AddNumbersService {
    /// New AddNumbersService
    pub fn new(bitvmx_service: Arc<BitvmxService>) -> Self {
        Self {
            game_store: Arc::new(AddNumbersStore::new()),
            bitvmx_service,
        }
    }

    /// Get the current game
    pub async fn get_current_game(&self) -> Result<Option<AddNumbersGame>, anyhow::Error> {
        let game = self
            .game_store
            .get_current_game()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get current game: {e:?}"))?;

        Ok(game)
    }

    /// Get the game
    pub async fn get_game(&self, id: Uuid) -> Result<Option<AddNumbersGame>, anyhow::Error> {
        let game = self
            .game_store
            .get_game(id)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get game: {e:?}"))?;

        Ok(game)
    }

    /// Setup the participants
    pub async fn setup_participants(
        &self,
        aggregated_id: Uuid,
        leader_idx: u16,
        participants_addresses: Vec<P2PAddress>,
        participants_keys: Vec<String>,
        role: PlayerRole,
    ) -> Result<(Uuid, PublicKey), anyhow::Error> {
        let p2p_addresses: Vec<BitVMXP2PAddress> = participants_addresses
            .iter()
            .map(|p2p| p2p.clone().into())
            .collect();

        // Validate the participants keys
        let public_keys = participants_keys
            .iter()
            .map(|key| {
                if key.is_empty() {
                    return Err(anyhow::anyhow!("Participants key cannot be empty"));
                }
                PublicKey::from_str(key).map_err(|_| anyhow::anyhow!("Invalid participants key"))
            })
            .collect::<Result<Vec<PublicKey>, anyhow::Error>>()?;

        // Create the aggregated key
        let aggregated_key = self
            .bitvmx_service
            .create_agregated_key(aggregated_id, p2p_addresses, Some(public_keys), leader_idx)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create aggregated key: {e:?}"))?;

        debug!("Aggregated key created: {:?}", aggregated_key);

        // Create the program id
        let program_id = Uuid::new_v5(&Uuid::NAMESPACE_OID, aggregated_id.as_bytes());
        debug!("🎉 Setting up game with program id: {:?}", program_id);

        // Setup the game
        self.game_store
            .setup_participants(
                program_id,
                aggregated_id,
                participants_addresses,
                participants_keys,
                aggregated_key,
                role,
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to setup game: {e:?}"))?;

        Ok((program_id, aggregated_key))
    }

    /// Place the bet
    pub async fn place_bet(
        &self,
        program_id: Uuid,
        amount: u64,
    ) -> Result<AddNumbersGame, anyhow::Error> {
        // Get the game
        let game = self
            .get_game(program_id)
            .await?
            .ok_or(anyhow::anyhow!("Game not found"))?;

        if game.status != AddNumbersGameStatus::PlaceBet {
            return Err(anyhow::anyhow!("Game is not in place bet state"));
        }

        if game.role == PlayerRole::Player2 {
            self.game_store
                .change_state(program_id, AddNumbersGameStatus::SetupFunding)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to update game state: {e:?}"))?;
            return Ok(game);
        }

        // Get the aggregated key
        let aggregated_key = game.bitvmx_program_properties.aggregated_key;

        // Get the protocol fees amount
        let protocol_amount = self.bitvmx_service.protocol_cost();
        // Preparer the utxo destination for the protocol fees
        let protocol_destination = self
            .game_store
            .protocol_destination(&aggregated_key, protocol_amount)?;

        // Prepare the utxo destination for the bet
        let bet_destination = self
            .game_store
            .protocol_destination(&aggregated_key, amount)?;

        // Send funds to cover protocol fees to the aggregated key
        let (funding_uuid, funding_txid) = self
            .bitvmx_service
            .send_funds(&Destination::Batch(vec![
                protocol_destination,
                bet_destination,
            ]))
            .await
            .map_err(|e| anyhow::anyhow!(format!("Failed to send protocol funds: {e:?}")))?;
        debug!(
            "Sent {protocol_amount} satoshis to cover protocol fees and bet {} satoshis to the aggregated key txid: {:?} uuid: {:?}",
            amount, funding_txid, funding_uuid
        );

        // Wait for the Transaction Status responses
        debug!("Waiting for transaction status responses");
        let funding_tx_status = self
            .bitvmx_service
            .wait_transaction_response(funding_uuid.to_string())
            .await
            .map_err(|e| {
                anyhow::anyhow!(format!(
                    "Failed to wait for transaction status response: {e:?}"
                ))
            })?;

        debug!(
            "Received transaction status responses for correlation ids: {:?} and {:?}",
            funding_uuid, funding_txid
        );

        if funding_tx_status.confirmations == 0 {
            error!(
                "Transaction {} not confirmed for correlation id: {:?}",
                funding_txid, funding_uuid
            );
            return Err(anyhow::anyhow!("Transaction not confirmed"));
        }
        // Protcol cost transaction
        self.game_store
            .set_dispute_tx(
                program_id,
                dispute::EXTERNAL_START.to_string(),
                funding_tx_status.clone(),
            )
            .await
            .map_err(|e| {
                anyhow::anyhow!(format!("Failed to set EXTERNAL_START dispute tx: {e:?}"))
            })?;

        // Player bet transaction
        self.game_store
            .set_dispute_tx(
                program_id,
                dispute::EXTERNAL_ACTION.to_string(),
                funding_tx_status,
            )
            .await
            .map_err(|e| {
                anyhow::anyhow!(format!("Failed to set EXTERNAL_START dispute tx: {e:?}"))
            })?;

        debug!("Protocol and bet transactions confirmed, marking funding UTXOs as mined");
        let protocol_leaves = self.game_store.protocol_scripts(&aggregated_key);

        let protocol_output_type =
            OutputType::taproot(protocol_amount, &aggregated_key, &protocol_leaves).map_err(
                |e| {
                    anyhow::anyhow!(format!(
                        "Failed to obtain protocol output type from aggregated key: {e:?}"
                    ))
                },
            )?;
        let bet_output_type = OutputType::taproot(amount, &aggregated_key, &protocol_leaves)
            .map_err(|e| {
                anyhow::anyhow!(format!(
                    "Failed to obtain protocol output type from aggregated key: {e:?}"
                ))
            })?;

        let funding_protocol_utxo: Utxo = Utxo {
            txid: funding_txid.to_string(),
            vout: 0,
            amount: protocol_amount,
            output_type: serde_json::to_value(protocol_output_type).map_err(|e| {
                anyhow::anyhow!(format!(
                    "Failed to convert protocol output type to JSON: {e:?}"
                ))
            })?,
        };
        let funding_bet_utxo: Utxo = Utxo {
            txid: funding_txid.to_string(),
            vout: 1,
            amount,
            output_type: serde_json::to_value(bet_output_type).map_err(|e| {
                anyhow::anyhow!(format!("Failed to convert bet output type to JSON: {e:?}"))
            })?,
        };

        // Save the funding UTXOs in AddNumbersService
        self.game_store
            .save_funding_utxos(
                program_id,
                funding_protocol_utxo.clone(),
                funding_bet_utxo.clone(),
            )
            .await
            .map_err(|e| anyhow::anyhow!(format!("Failed to save my funding UTXO: {e:?}")))?;

        debug!("Saved my funding UTXOs in AddNumbersService");

        Ok(game)
    }

    /// Setup the funding UTXOs
    pub async fn setup_funding_utxo(
        &self,
        program_id: Uuid,
        funding_protocol_utxo: Utxo,
        funding_bet_utxo: Utxo,
    ) -> Result<(), anyhow::Error> {
        // Save the funding UTXOs
        self.game_store
            .save_funding_utxos(
                program_id,
                funding_protocol_utxo.clone(),
                funding_bet_utxo.clone(),
            )
            .await
            .map_err(|e| anyhow::anyhow!(format!("Failed to add funding UTXO: {e:?}")))?;

        // For now we use the same transaction for the protocol cost and the player bet with different vouts
        if funding_protocol_utxo.txid != funding_bet_utxo.txid {
            return Err(anyhow::anyhow!(
                "Protocol and bet UTXOs should have the same transaction ID at this moment"
            ));
        }

        // Get the funding transaction status
        let funding_tx_status = self
            .bitvmx_service
            .get_transaction(funding_protocol_utxo.txid)
            .await
            .map_err(|e| anyhow::anyhow!(format!("Failed to get transaction: {e:?}")))?;

        // Protcol cost transaction
        self.game_store
            .set_dispute_tx(
                program_id,
                dispute::EXTERNAL_START.to_string(),
                funding_tx_status.clone(),
            )
            .await
            .map_err(|e| {
                anyhow::anyhow!(format!("Failed to set EXTERNAL_START dispute tx: {e:?}"))
            })?;

        // Player bet transaction
        self.game_store
            .set_dispute_tx(
                program_id,
                dispute::EXTERNAL_ACTION.to_string(),
                funding_tx_status,
            )
            .await
            .map_err(|e| {
                anyhow::anyhow!(format!("Failed to set EXTERNAL_START dispute tx: {e:?}"))
            })?;

        Ok(())
    }

    /// Setup the game
    #[instrument(name = "setup_game", skip(self, worker_service))]
    pub async fn setup_game(
        &self,
        program_id: Uuid,
        number1: u32,
        number2: u32,
        worker_service: Arc<WorkerService>,
    ) -> Result<(), anyhow::Error> {
        // Get the game
        let game = self
            .get_game(program_id)
            .await?
            .ok_or(anyhow::anyhow!("Game not found"))?
            .clone();

        // Set inputs values, Concatenate the two input numbers as bytes
        let mut concatenated_bytes = Vec::<u8>::new();
        concatenated_bytes.extend_from_slice(&number1.to_be_bytes());
        concatenated_bytes.extend_from_slice(&number2.to_be_bytes());

        // Set all necesary program variables in BitVMX

        // Set program input 0, the two numbers to sum
        self.bitvmx_service
            .set_program_input(program_id, 0, concatenated_bytes)
            .await
            .map_err(|e| anyhow::anyhow!(format!("Failed to set program input: {e:?}")))?;

        // Set aggregated key
        self.bitvmx_service
            .set_variable(
                program_id,
                "aggregated",
                VariableTypes::PubKey(game.bitvmx_program_properties.aggregated_key),
            )
            .await
            .map_err(|e| {
                anyhow::anyhow!(format!("Failed to set variable aggregated pubkey: {e:?}"))
            })?;

        // Set protocol cost utxo
        let protocol_utxo = game
            .bitvmx_program_properties
            .funding_protocol_utxo
            .ok_or(anyhow::anyhow!("Protocol UTXO not found"))?;

        self.bitvmx_service
            .set_variable(
                program_id,
                "utxo",
                VariableTypes::Utxo(protocol_utxo.into()),
            )
            .await
            .map_err(|e| anyhow::anyhow!(format!("Failed to set variable protocol utxo: {e:?}")))?;

        // Set bet utxo
        let bet_utxo = game
            .bitvmx_program_properties
            .funding_bet_utxo
            .ok_or(anyhow::anyhow!("Bet UTXO not found"))?;

        self.bitvmx_service
            .set_variable(
                program_id,
                "utxo_prover_win_action",
                VariableTypes::Utxo(bet_utxo.into()),
            )
            .await
            .map_err(|e| anyhow::anyhow!(format!("Failed to set variable bet utxo: {e:?}")))?;

        // Set program definition file path
        self.bitvmx_service
            .set_variable(
                program_id,
                "program_definition",
                VariableTypes::String(PROGRAM_PATH.to_string()),
            )
            .await
            .map_err(|e| {
                anyhow::anyhow!(format!("Failed to set variable program definition: {e:?}"))
            })?;

        // Set timelock blocks
        self.bitvmx_service
            .set_variable(
                program_id,
                dispute::TIMELOCK_BLOCKS_KEY,
                VariableTypes::Number(dispute::TIMELOCK_BLOCKS.into()),
            )
            .await
            .map_err(|e| {
                anyhow::anyhow!(format!("Failed to set variable timelock blocks: {e:?}"))
            })?;

        // Get the participants addresses
        let participants_addresses: Vec<BitVMXP2PAddress> = game
            .bitvmx_program_properties
            .participants_addresses
            .iter()
            .map(|p2p| p2p.clone().into())
            .collect();

        // Setup program in BitVMX
        self.bitvmx_service
            .program_setup(
                program_id,
                bitvmx_client::types::PROGRAM_TYPE_DRP,
                participants_addresses,
                1,
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to set variable program setup: {e:?}"))?;

        // Set game as started
        self.game_store
            .setup_game(program_id, number1, number2)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to save start game state: {e:?}"))?;

        if game.role == PlayerRole::Player2 {
            // Player 1 will send the challenge transaction to start the game.
            // Player 2 will wait until see the first challenge transaction.
            worker_service
                .handle_start_game_tx(program_id)
                .map_err(|e| {
                    anyhow::anyhow!("Failed to enqueue job to wait for start game: {e:?}")
                })?;
        }
        Ok(())
    }

    /// Start the game
    /// Player 1 will send the challenge transaction to start the game.
    #[instrument(name = "start_game", skip(self, worker_service))]
    pub async fn start_game(
        &self,
        program_id: Uuid,
        worker_service: Arc<WorkerService>,
    ) -> Result<(String, TransactionStatus), anyhow::Error> {
        // Get the game
        let game = self
            .get_game(program_id)
            .await?
            .ok_or(anyhow::anyhow!("Game not found"))?;

        if game.status != AddNumbersGameStatus::StartGame {
            return Err(anyhow::anyhow!("Game is not in start game state"));
        }

        if game.role != PlayerRole::Player1 {
            return Err(anyhow::anyhow!(
                "Invalid game role, only player 1 can start the game"
            ));
        }

        // Player 1 send the challenge transaction to start the game.
        let (challenge_tx_name, challenge_tx) = self
            .bitvmx_service
            .start_challenge(program_id)
            .await
            .map_err(|e| anyhow::anyhow!(format!("Failed to start challenge: {e:?}")))?;

        // Set the game as setup
        self.game_store
            .start_game(program_id, challenge_tx_name.clone(), &challenge_tx)
            .await
            .map_err(|e| anyhow::anyhow!(format!("Failed to set game as started: {e:?}")))?;

        // Player 2 will make the guess
        // Player 1 will wait until see the game result.
        worker_service
            .handle_player2_wins_game_outcome_tx(program_id)
            .map_err(|e| {
                anyhow::anyhow!("Failed to enqueue job to wait for player 2 to win the game: {e:?}")
            })?;

        Ok((challenge_tx_name, challenge_tx))
    }

    /// Wait for the other player to start the game
    /// Player 2 will wait until see the first challenge transaction.
    #[instrument(name = "wait_start_game_tx", skip(self))]
    pub async fn wait_start_game_tx(&self, program_id: Uuid) -> Result<(), anyhow::Error> {
        debug!("Waiting for other player to start the game");
        let (challenge_tx_name, challenge_tx) = self
            .bitvmx_service
            .wait_transaction_by_name_response(program_id, dispute::START_CH)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to wait for start game: {e:?}"))?;
        debug!("Other player started the game");

        // Set the game as setuped
        self.game_store
            .start_game(program_id, challenge_tx_name.clone(), &challenge_tx)
            .await
            .map_err(|e| anyhow::anyhow!(format!("Failed to set game as started: {e:?}")))?;

        Ok(())
    }

    /// Submit the sum
    /// Player 2 will send the sum to answer the challenge.
    #[instrument(name = "submit_sum", skip(self))]
    pub async fn submit_sum(
        &self,
        program_id: Uuid,
        guess: u32,
    ) -> Result<AddNumbersGame, anyhow::Error> {
        // Store the submitted sum
        self.game_store
            .make_guess(program_id, guess)
            .await
            .map_err(|e| anyhow::anyhow!(format!("Failed to store submitted sum: {e:?}")))?;

        // The input index is 1 because the first input is the numbers to sum
        let input_index = 1;

        // Player 2 sets the input transaction with the sum in BitVMX
        self.bitvmx_service
            .set_program_input(program_id, input_index, guess.to_be_bytes().to_vec())
            .await
            .map_err(|e| anyhow::anyhow!(format!("Failed to set program input: {e:?}")))?;

        // Send the input transaction to BitVMX
        let (challenge_input_tx, challenge_input_tx_name) = self
            .bitvmx_service
            .send_transaction_by_name(
                program_id,
                BitvmxService::dispute_input_tx_name(input_index).as_str(),
            )
            .await
            .map_err(|e| anyhow::anyhow!(format!("Failed to send challenge input: {e:?}")))?;
        debug!(
            "Challenge input transaction: {:?}",
            challenge_input_tx.tx_id
        );

        // Set the challenge input transaction
        self.game_store
            .set_dispute_tx(program_id, challenge_input_tx_name, challenge_input_tx)
            .await
            .map_err(|e| anyhow::anyhow!(format!("Failed to set challenge tx: {e:?}")))?;

        // Wait for the dispute transactions to be confirmed
        let dispute_result = self.wait_dispute_transactions(program_id).await;

        if let Err(e) = dispute_result {
            if !e.to_string().contains("Request timed out") {
                return Err(anyhow::anyhow!(
                    "Failed to wait for dispute transactions: {e:?}"
                ));
            }

            info!("Player 1 won the game after timeout: {e:?}");
            // Player 1 wins the game. Set the game as complete
            let game = self
                .game_store
                .set_game_complete(program_id, GameOutcome::Lose, GameReason::Challenge)
                .await
                .map_err(|e| anyhow::anyhow!(format!("Failed to set game complete: {e:?}")))?;

            return Ok(game);
        }

        info!("Player 2 won the game");
        // Player 2 wins the game. Update the game status when you know the outcome.
        let game = self
            .game_store
            .set_game_complete(program_id, GameOutcome::Win, GameReason::Challenge)
            .await
            .map_err(|e| anyhow::anyhow!(format!("Failed to set game complete: {e:?}")))?;

        Ok(game)
    }

    async fn wait_dispute_transactions(&self, program_id: Uuid) -> Result<(), anyhow::Error> {
        debug!("Waiting for dispute transactions to be confirmed");
        let mut join_set = JoinSet::new();
        self.spawn_wait_task_transaction_by_name(&mut join_set, program_id, dispute::COMMITMENT);
        self.spawn_wait_task_transaction_by_name(&mut join_set, program_id, "NARY_PROVER_1");
        self.spawn_wait_task_transaction_by_name(&mut join_set, program_id, "NARY_VERIFIER_1");
        self.spawn_wait_task_transaction_by_name(&mut join_set, program_id, "NARY_PROVER_2");
        self.spawn_wait_task_transaction_by_name(&mut join_set, program_id, "NARY_VERIFIER_2");
        self.spawn_wait_task_transaction_by_name(&mut join_set, program_id, dispute::EXECUTE);
        self.spawn_wait_task_transaction_by_name(
            &mut join_set,
            program_id,
            format!("{}_START", dispute::PROVER_WINS).as_str(),
        );
        self.spawn_wait_task_transaction_by_name(
            &mut join_set,
            program_id,
            format!("{}_SUCCESS", dispute::PROVER_WINS).as_str(),
        );
        self.spawn_wait_task_transaction_by_name(
            &mut join_set,
            program_id,
            dispute::ACTION_PROVER_WINS,
        );

        // Wait until you know the result of the game
        while let Some(res) = join_set.join_next().await {
            match res {
                Ok(result) => match result {
                    Ok((tx_name, tx_status)) => {
                        self.game_store
                            .set_dispute_tx(program_id, tx_name, tx_status)
                            .await
                            .map_err(|e| {
                                anyhow::anyhow!(format!("Failed to set dispute tx: {e:?}"))
                            })?;
                    }
                    Err(e) => {
                        return Err(anyhow::anyhow!("Wait transaction by name failed: {:?}", e))
                    }
                },
                Err(e) => return Err(anyhow::anyhow!("Wait transaction by name failed: {:?}", e)),
            }
        }

        debug!("All dispute transactions confirmed");

        Ok(())
    }

    /// Helper function to spawn a wait task for transaction by name
    fn spawn_wait_task_transaction_by_name(
        &self,
        join_set: &mut JoinSet<Result<(String, TransactionStatus), anyhow::Error>>,
        program_id: Uuid,
        tx_name: &str,
    ) {
        let bitvmx_service = self.bitvmx_service.clone();
        let tx_name = tx_name.to_string();
        join_set.spawn(async move {
            bitvmx_service
                .wait_transaction_by_name_response(program_id, &tx_name)
                .await
        });
    }

    /// Wait for other player to win the game
    /// Player 1 will wait until see the first challenge transaction.
    #[instrument(name = "wait_player2_wins_game_outcome_tx", skip(self))]
    pub async fn wait_player2_wins_game_outcome_tx(
        &self,
        program_id: Uuid,
    ) -> Result<(), anyhow::Error> {
        debug!("Waiting for player 2 to win the game");

        // Wait challenge input transaction
        self.bitvmx_service
            .wait_transaction_by_name_response(
                program_id,
                BitvmxService::dispute_input_tx_name(1).as_str(),
            )
            .await
            .map_err(|e| anyhow::anyhow!(format!("Failed to wait for challenge input: {e:?}")))?;

        let dispute_result = self.wait_dispute_transactions(program_id).await;
        if let Err(e) = dispute_result {
            if !e.to_string().contains("Request timed out") {
                return Err(anyhow::anyhow!(
                    "Failed to wait for dispute transactions: {e:?}"
                ));
            }

            info!("Player 1 won the game after timeout: {e:?}");
            // Player 1 wins the game. Set the game as complete
            self.game_store
                .set_game_complete(program_id, GameOutcome::Win, GameReason::Challenge)
                .await
                .map_err(|e| anyhow::anyhow!(format!("Failed to set game complete: {e:?}")))?;

            return Ok(());
        }

        info!("Player 2 won the game");
        // Player 2 wins the game. Set the game as complete
        self.game_store
            .set_game_complete(program_id, GameOutcome::Lose, GameReason::Challenge)
            .await
            .map_err(|e| anyhow::anyhow!(format!("Failed to set game complete: {e:?}")))?;

        Ok(())
    }
}
