use crate::models::{
    AddNumbersGame, AddNumbersGameStatus, BitVMXProgramProperties, GameOutcome, GameReason,
    P2PAddress, PlayerRole, Utxo,
};
use crate::utils::bitcoin;
use bitvmx_client::bitcoin::{Address, PublicKey};
use bitvmx_client::bitvmx_wallet::wallet::Destination;
use bitvmx_client::protocol_builder::scripts::{self, ProtocolScript};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug)]
pub struct AddNumbersService {
    games: Arc<RwLock<HashMap<Uuid, AddNumbersGame>>>,
}

impl AddNumbersService {
    pub fn new() -> Self {
        Self {
            games: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn setup_participants(
        &self,
        program_id: Uuid,
        aggregated_id: Uuid,
        participants_addresses: Vec<P2PAddress>,
        participants_keys: Vec<String>,
        aggregated_key: PublicKey,
        role: PlayerRole,
    ) -> Result<AddNumbersGame, anyhow::Error> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let protocol_address = self.protocol_address(&aggregated_key)?.to_string();

        let game = AddNumbersGame {
            program_id,
            number1: None,
            number2: None,
            guess: None,
            status: AddNumbersGameStatus::PlaceBet,
            created_at: now,
            updated_at: now,
            role,
            bitvmx_program_properties: BitVMXProgramProperties {
                aggregated_key,
                aggregated_id,
                protocol_address,
                participants_addresses,
                participants_keys,
                funding_protocol_utxo: None,
                funding_bet_utxo: None,
            },
        };

        let mut hash_map = self
            .games
            .write()
            .map_err(|e| anyhow::anyhow!("Failed to write to games: {e:?}"))?;
        if hash_map.contains_key(&program_id) {
            return Err(anyhow::anyhow!("Game already exists"));
        }
        hash_map.insert(program_id, game.clone());

        Ok(game)
    }

    pub fn get_game(&self, id: Uuid) -> Result<Option<AddNumbersGame>, anyhow::Error> {
        let hash_map = self
            .games
            .read()
            .map_err(|e| anyhow::anyhow!("Failed to read from games: {e:?}"))?;
        Ok(hash_map.get(&id).cloned())
    }

    pub fn get_current_game_id(&self) -> Result<Option<AddNumbersGame>, anyhow::Error> {
        let hash_map = self
            .games
            .read()
            .map_err(|e| anyhow::anyhow!("Failed to read from games: {e:?}"))?;
        Ok(hash_map
            .iter()
            .find(|(_, game)| game.status != AddNumbersGameStatus::Finished)
            .map(|(_, game)| game.clone()))
    }

    /// Save the funding utxos for the current participant
    pub fn save_funding_utxos(
        &self,
        program_id: Uuid,
        funding_protocol_utxo: Utxo,
        funding_bet_utxo: Utxo,
    ) -> Result<(), anyhow::Error> {
        let mut hash_map = self
            .games
            .write()
            .map_err(|e| anyhow::anyhow!("Failed to write to games: {e:?}"))?;
        let game = hash_map
            .get_mut(&program_id)
            .ok_or(anyhow::anyhow!("Game not found"))?;

        // Validate the game status
        if game.status != AddNumbersGameStatus::PlaceBet {
            return Err(anyhow::anyhow!("Game is not in place bet state"));
        }

        // Save the funding bet UTXO
        game.bitvmx_program_properties.funding_bet_utxo = Some(funding_bet_utxo);
        game.bitvmx_program_properties.funding_protocol_utxo = Some(funding_protocol_utxo);
        game.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Update the game status
        game.status = AddNumbersGameStatus::PlaceBet;

        Ok(())
    }

    pub fn change_state(
        &self,
        program_id: Uuid,
        status: AddNumbersGameStatus,
    ) -> Result<(), anyhow::Error> {
        let mut hash_map = self
            .games
            .write()
            .map_err(|e| anyhow::anyhow!("Failed to write to games: {e:?}"))?;
        let game = hash_map
            .get_mut(&program_id)
            .ok_or(anyhow::anyhow!("Game not found"))?;

        // Update the game status
        game.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        game.status = status;

        Ok(())
    }

    pub fn protocol_scripts(&self, aggregated_key: &PublicKey) -> Vec<ProtocolScript> {
        // Todo check if this tap leaves are correct
        vec![
            scripts::check_aggregated_signature(aggregated_key, scripts::SignMode::Aggregate),
            scripts::check_aggregated_signature(aggregated_key, scripts::SignMode::Aggregate),
        ]
    }

    pub fn protocol_destination(
        &self,
        aggregated_key: &PublicKey,
        amount: u64,
    ) -> Result<Destination, anyhow::Error> {
        // Get the aggregated key and protocol information
        let x_only_pubkey = bitcoin::pub_key_to_xonly(aggregated_key).map_err(|e| {
            anyhow::anyhow!("Failed to convert aggregated key to x only pubkey: {e:?}")
        })?;
        let tap_leaves = self.protocol_scripts(aggregated_key);
        let destination = Destination::P2TR(x_only_pubkey, tap_leaves, amount);
        Ok(destination)
    }

    pub fn protocol_address(&self, aggregated_key: &PublicKey) -> Result<Address, anyhow::Error> {
        // Todo check if this tap leaves are correct
        let x_only_pubkey = bitcoin::pub_key_to_xonly(aggregated_key).map_err(|e| {
            anyhow::anyhow!("Failed to convert aggregated key to x only pubkey: {e:?}")
        })?;
        let tap_leaves = self.protocol_scripts(aggregated_key);
        let p2tr_address = bitcoin::pub_key_to_p2tr(&x_only_pubkey, &tap_leaves).map_err(|e| {
            anyhow::anyhow!("Failed to convert aggregated key to p2tr address: {e:?}")
        })?;
        Ok(p2tr_address)
    }

    pub fn setup_game(
        &self,
        program_id: Uuid,
        number1: u32,
        number2: u32,
    ) -> Result<(), anyhow::Error> {
        let mut hash_map = self
            .games
            .write()
            .map_err(|e| anyhow::anyhow!("Failed to write to games: {e:?}"))?;
        let game = hash_map
            .get_mut(&program_id)
            .ok_or(anyhow::anyhow!("Game not found"))?;

        // Validate the game status
        if game.status != AddNumbersGameStatus::SetupGame {
            return Err(anyhow::anyhow!("Game is not in start game state"));
        }

        game.number1 = Some(number1);
        game.number2 = Some(number2);
        game.status = AddNumbersGameStatus::StartGame;
        Ok(())
    }

    pub fn start_game(&self, program_id: Uuid) -> Result<(), anyhow::Error> {
        let mut hash_map = self
            .games
            .write()
            .map_err(|e| anyhow::anyhow!("Failed to write to games: {e:?}"))?;
        let game = hash_map
            .get_mut(&program_id)
            .ok_or(anyhow::anyhow!("Game not found"))?;

        // TODO PEDRO: Here you have to :
        // Player 1 send the challenge transaction to start the game.
        // Player 2 will wait until see the first challenge transaction.

        game.status = AddNumbersGameStatus::SubmitGameData;
        game.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Ok(())
    }

    pub fn make_guess(&self, id: Uuid, guess: u32) -> Result<AddNumbersGame, anyhow::Error> {
        let mut hash_map = self
            .games
            .write()
            .map_err(|e| anyhow::anyhow!("Failed to write to games: {e:?}"))?;
        let game = hash_map
            .get_mut(&id)
            .ok_or(anyhow::anyhow!("Game not found"))?;

        // Validate game status
        if game.status != AddNumbersGameStatus::SubmitGameData {
            return Err(anyhow::anyhow!("Game is not in waiting for guess state"));
        }

        // Make the guess
        game.guess = Some(guess);
        game.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Update the game status
        game.status = AddNumbersGameStatus::GameComplete {
            outcome: GameOutcome::Win,
            reason: GameReason::Accept,
        };

        Ok(game.clone())
    }
}

impl Default for AddNumbersService {
    fn default() -> Self {
        Self::new()
    }
}
