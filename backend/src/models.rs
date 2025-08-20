use crate::types::{Game, GameStatus, Move, Player, Position};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub struct GameStore {
    games: HashMap<Uuid, Game>,
}

impl GameStore {
    pub fn new() -> Self {
        Self {
            games: HashMap::new(),
        }
    }

    pub fn create_game(&mut self) -> Game {
        let id = Uuid::new_v4();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let game = Game {
            id,
            board: [[None; 3]; 3],
            current_player: Player::X,
            status: GameStatus::Waiting,
            moves: Vec::new(),
            created_at: now,
            updated_at: now,
        };

        self.games.insert(id, game.clone());
        game
    }

    pub fn get_game(&self, id: Uuid) -> Option<&Game> {
        self.games.get(&id)
    }

    pub fn make_move(&mut self, id: Uuid, player: Player, position: Position) -> Result<Game, String> {
        let game = self.games.get_mut(&id).ok_or("Game not found")?;
        
        // Validate move
        if game.status != GameStatus::InProgress && game.status != GameStatus::Waiting {
            return Err("Game is already finished".to_string());
        }

        if game.current_player != player {
            return Err("Not your turn".to_string());
        }

        if position.row >= 3 || position.col >= 3 {
            return Err("Invalid position".to_string());
        }

        if game.board[position.row as usize][position.col as usize].is_some() {
            return Err("Position already occupied".to_string());
        }

        // Make the move
        game.board[position.row as usize][position.col as usize] = Some(player);
        game.moves.push(Move { player, position });
        game.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Update game status
        if game.status == GameStatus::Waiting {
            game.status = GameStatus::InProgress;
        }

        // Check for win
        let board = game.board;
        if let Some(winner) = Self::check_winner(&board) {
            game.status = GameStatus::Won { winner };
        } else if Self::is_board_full(&board) {
            game.status = GameStatus::Draw;
        } else {
            // Switch players
            game.current_player = match game.current_player {
                Player::X => Player::O,
                Player::O => Player::X,
            };
        }

        Ok(game.clone())
    }

    fn check_winner(board: &[[Option<Player>; 3]; 3]) -> Option<Player> {
        // Check rows
        for row in board.iter() {
            if let Some(player) = row[0] {
                if row.iter().all(|&cell| cell == Some(player)) {
                    return Some(player);
                }
            }
        }

        // Check columns
        for col in 0..3 {
            if let Some(player) = board[0][col] {
                if (0..3).all(|row| board[row][col] == Some(player)) {
                    return Some(player);
                }
            }
        }

        // Check diagonals
        if let Some(player) = board[0][0] {
            if board[0][0] == Some(player) && board[1][1] == Some(player) && board[2][2] == Some(player) {
                return Some(player);
            }
        }

        if let Some(player) = board[0][2] {
            if board[0][2] == Some(player) && board[1][1] == Some(player) && board[2][0] == Some(player) {
                return Some(player);
            }
        }

        None
    }

    fn is_board_full(board: &[[Option<Player>; 3]; 3]) -> bool {
        board.iter().all(|row| row.iter().all(|cell| cell.is_some()))
    }
}

impl Default for GameStore {
    fn default() -> Self {
        Self::new()
    }
}
