use crate::models::{AddNumbersGame, AddNumbersGameStatus, BitVMXProgramProperties};
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

    pub fn create_game(&mut self, id: Uuid, number1: i32, number2: i32) -> AddNumbersGame {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let game = AddNumbersGame {
            id,
            number1,
            number2,
            guess: None,
            status: AddNumbersGameStatus::CreateProgram,
            created_at: now,
            updated_at: now,
            bitvmx_program_properties: BitVMXProgramProperties {
                aggregated_key: None,
                aggregated_key_uuid: Uuid::default(),
                participants: vec![],
                participants_keys: vec![],
                leader_idx: 0,
                my_idx: 0,
                initial_utxo: None,
                player1_bet_utxo: None,
                player2_bet_utxo: None,
            },
        };

        self.games.insert(id, game.clone());
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
}

impl Default for AddNumbersService {
    fn default() -> Self {
        Self::new()
    }
}
