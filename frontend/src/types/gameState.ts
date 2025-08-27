export enum GameState {
  Setup = "setup",
  WaitingPeer = "waitingPeer",
  WaitingResponse = "waitingResponse",
  ResponseReceived = "responseReceived",
  SetupCompleted = "setupComplete",
  Disputed = "disputed",
  Timeout = "timeout",
}
