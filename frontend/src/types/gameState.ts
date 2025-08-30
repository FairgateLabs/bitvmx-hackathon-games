export enum GameState {
  ChooseGame = "ChooseGame",
  SetupNetwork = "SetupNetwork",
  ChooseRole = "ChooseRole",
  // SetupFunding = "SetupFunding", This is only in testnet
  SetupConnection = "SetupConnection",
  SetupProgram = "SetupProgram",
  StartGame = "StartGame",
  ChooseAction = "ChooseAction",
  ChallengeAnswer = "ChallengeAnswer",
  GameCompleteYouLose = "GameCompleteYouLose",
  GameCompleteYouWin = "GameCompleteYouWin",
  TransferFunds = "TransferFunds",
}

export interface GameNumbersToAdd {
  number1?: number;
  number2?: number;
}
