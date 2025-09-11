use crate::models::{
    AddNumbersGame, AddNumbersGameStatus, BitVMXProgramProperties, P2PAddress, PlayerRole, Utxo,
};
use crate::utils::bitcoin;
use bitvmx_client::bitcoin::{Address, PublicKey};
use bitvmx_client::protocol_builder::scripts::{self, ProtocolScript};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug)]
pub struct AddNumbersService {
    games: HashMap<Uuid, AddNumbersGame>,
}

impl AddNumbersService {
    pub fn new() -> Self {
        Self {
            games: HashMap::new(),
        }
    }

    pub fn setup_game(
        &mut self,
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
            status: AddNumbersGameStatus::SetupParticipants,
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

        self.games.insert(program_id, game.clone());
        Ok(game)
    }

    pub fn get_game(&self, id: Uuid) -> Option<&AddNumbersGame> {
        self.games.get(&id)
    }

    pub fn make_guess(&mut self, id: Uuid, guess: i32) -> Result<AddNumbersGame, String> {
        let game = self.games.get_mut(&id).ok_or("Game not found")?;

        // Validate game status
        if game.status != AddNumbersGameStatus::SubmitSum {
            return Err("Game is not in waiting for guess state".to_string());
        }

        // Make the guess
        game.guess = Some(guess);
        game.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        game.status = AddNumbersGameStatus::SubmitSum;

        Ok(game.clone())
    }

    pub fn get_current_game_id(&self) -> Option<AddNumbersGame> {
        self.games
            .iter()
            .find(|(_, game)| game.status != AddNumbersGameStatus::Finished)
            .map(|(_, game)| game.clone())
    }

    pub fn save_fundings_utxos(
        &mut self,
        program_id: Uuid,
        funding_protocol_utxo: Utxo,
        funding_bet_utxo: Utxo,
    ) -> Result<(), String> {
        let game = self.games.get_mut(&program_id).ok_or("Game not found")?;

        // Save the funding bet UTXO
        game.bitvmx_program_properties.funding_bet_utxo = Some(funding_bet_utxo);
        game.bitvmx_program_properties.funding_protocol_utxo = Some(funding_protocol_utxo);

        Ok(())
    }

    pub fn protocol_scripts(&self,aggregated_key: &PublicKey) -> Vec<ProtocolScript> {
        // Todo check if this tap leaves are correct
        vec![
            scripts::check_aggregated_signature(aggregated_key, scripts::SignMode::Aggregate),
            scripts::check_aggregated_signature(aggregated_key, scripts::SignMode::Aggregate),
        ]
    }
    
    pub fn protocol_address(&self, aggregated_key: &PublicKey) -> Result<Address, anyhow::Error> {
        // Todo check if this tap leaves are correct
        let x_only_pubkey = bitcoin::pub_key_to_xonly(aggregated_key)
            .map_err(|e| anyhow::anyhow!("Failed to convert aggregated key to x only pubkey: {e:?}"))?;
        let tap_leaves = self.protocol_scripts(aggregated_key);
        let p2tr_address = bitcoin::pub_key_to_p2tr(&x_only_pubkey, &tap_leaves)
            .map_err(|e| anyhow::anyhow!("Failed to convert aggregated key to p2tr address: {e:?}"))?;
        Ok(p2tr_address)
    }
}

impl Default for AddNumbersService {
    fn default() -> Self {
        Self::new()
    }
}
