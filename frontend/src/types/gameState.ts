export enum GameState {
  SetupNetwork = "SetupNetwork",
  SetupFunding = "SetupFunding",
  SetupConnection = "SetupConnection",
  SetupProgram = "SetupProgram",
  StartGame = "StartGame",
  AnswerStep = "AnswerStep",
  ChallengeStep = "ChallengeStep",
  GameCompleted = "GameCompleted",
}

export interface GameNumbersToAdd {
  number1?: number;
  number2?: number;
}
