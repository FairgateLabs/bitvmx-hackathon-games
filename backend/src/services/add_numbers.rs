use crate::models::{
    AddNumbersGame, AddNumbersGameStatus, BitVMXProgramProperties, P2PAddress, Utxo,
};
use bitvmx_client::bitcoin::PublicKey;
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
    ) -> AddNumbersGame {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let game = AddNumbersGame {
            program_id,
            number1: None,
            number2: None,
            guess: None,
            status: AddNumbersGameStatus::SetupParticipants,
            created_at: now,
            updated_at: now,
            bitvmx_program_properties: BitVMXProgramProperties {
                aggregated_key,
                aggregated_id,
                participants_addresses,
                participants_keys,
                funding_protocol_utxo: None,
                funding_bet_utxo: None, // TODO PEDRO: My funding va aca, funding utxo when is mined.
            },
        };

        self.games.insert(program_id, game.clone());
        game
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
}

impl Default for AddNumbersService {
    fn default() -> Self {
        Self::new()
    }
}
