// TypeScript types for the Tic-Tac-Toe API
// Generated manually based on Rust types

export enum Player {
  X = "X",
  O = "O",
}

export enum GameStatus {
  Waiting = "Waiting",
  InProgress = "InProgress",
  Won = "Won",
  Draw = "Draw",
}

export interface Position {
  row: number;
  col: number;
}

export interface Move {
  player: Player;
  position: Position;
}

export interface Game {
  id: string;
  board: (Player | null)[][];
  current_player: Player;
  status: GameStatus | { Won: { winner: Player } };
  moves: Move[];
  created_at: string;
  updated_at: string;
}

export interface CreateGameRequest {
  player_name: string;
}

export interface CreateGameResponse {
  game: Game;
  message: string;
}

export interface MakeMoveRequest {
  player: Player;
  position: Position;
}

export interface MakeMoveResponse {
  game: Game;
  message: string;
}

export interface GameResponse {
  game: Game;
}

export interface GameStatusResponse {
  status: GameStatus | { Won: { winner: Player } };
  current_player?: Player;
}

export interface ErrorResponse {
  error: string;
  message: string;
}

export interface HealthResponse {
  status: string;
  timestamp: string;
}
