export enum GameState {
  Setup = "setup",
  WaitingPeer = "waitingPeer",
  WaitingResponse = "waitingResponse",
  ResponseReceived = "responseReceived",
  SetupCompleted = "setupComplete",
  Disputed = "disputed",
  Timeout = "timeout",
}

export interface GameNumbersToAdd {
  number1?: number;
  number2?: number;
}
