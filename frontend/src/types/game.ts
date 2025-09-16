export interface GameNumbersToAdd {
  number1?: number;
  number2?: number;
}

export enum EnumPlayerRole {
  Player1 = "Player1",
  Player2 = "Player2",
}

export type PlayerRole = "Player1" | "Player2";
export type GameOutcome = "Win" | "Lose";
export type GameReason = "Challenge" | "Timeout";
