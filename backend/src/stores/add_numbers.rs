use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use crate::types::{AddNumbersGame, AddNumbersGameStatus};

pub struct AddNumbersStore {
    games: HashMap<Uuid, AddNumbersGame>,
}

impl AddNumbersStore {
    pub fn new() -> Self {
        Self {
            games: HashMap::new(),
        }
    }

    pub fn create_game(&mut self, player1: String, player2: String) -> AddNumbersGame {
        let id = Uuid::new_v4();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let game = AddNumbersGame {
            id,
            player1,
            player2,
            number1: None,
            number2: None,
            guess: None,
            status: AddNumbersGameStatus::WaitingForNumbers,
            created_at: now,
            updated_at: now,
        };

        self.games.insert(id, game.clone());
        game
    }

    pub fn get_game(&self, id: Uuid) -> Option<&AddNumbersGame> {
        self.games.get(&id)
    }

    pub fn add_numbers(&mut self, id: Uuid, player: String, number1: i32, number2: i32) -> Result<AddNumbersGame, String> {
        let game = self.games.get_mut(&id).ok_or("Game not found")?;
        
        // Validate it's the correct player's turn
        if game.player1 != player {
            return Err("Not your turn to add numbers".to_string());
        }

        // Validate game status
        if game.status != AddNumbersGameStatus::WaitingForNumbers {
            return Err("Game is not in waiting for numbers state".to_string());
        }

        // Add the numbers
        game.number1 = Some(number1);
        game.number2 = Some(number2);
        game.status = AddNumbersGameStatus::WaitingForGuess;
        game.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Ok(game.clone())
    }

    pub fn make_guess(&mut self, id: Uuid, player: String, guess: i32) -> Result<AddNumbersGame, String> {
        let game = self.games.get_mut(&id).ok_or("Game not found")?;
        
        // Validate it's the correct player's turn
        if game.player2 != player {
            return Err("Not your turn to guess".to_string());
        }

        // Validate game status
        if game.status != AddNumbersGameStatus::WaitingForGuess {
            return Err("Game is not in waiting for guess state".to_string());
        }

        // Validate numbers are set
        let number1 = game.number1.ok_or("Numbers not set yet")?;
        let number2 = game.number2.ok_or("Numbers not set yet")?;
        let correct_answer = number1 + number2;

        // Make the guess
        game.guess = Some(guess);
        game.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Check if guess is correct
        if guess == correct_answer {
            game.status = AddNumbersGameStatus::Won { winner: player };
        } else {
            game.status = AddNumbersGameStatus::Lost { correct_answer };
        }

        Ok(game.clone())
    }
}

impl Default for AddNumbersStore {
    fn default() -> Self {
        Self::new()
    }
}
