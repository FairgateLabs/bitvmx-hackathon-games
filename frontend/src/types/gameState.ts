export enum GameStatePlayer1 {
  ChooseGame = "ChooseGame",
  ChooseNetwork = "SetupNetwork",
  ChooseRole = "ChooseRole",
  // SetupFunding = "SetupFunding", This is only in testnet
  SetupConnection = "SetupConnection",
  SetupProgram = "SetupProgram",
  StartGame = "StartGame",
  WaitingAnswer = "WaitingAnswer",
  ChooseAction = "ChooseAction",
  GameCompleteYouLoseByChallenge = "GameCompleteYouLoseByChallenge",
  GameCompleteYouLoseByTimeout = "GameCompleteYouLoseByTimeout",
  GameCompleteYouLoseByAccept = "GameCompleteYouLoseByAccept",
  GameCompleteYouWinByChallenge = "GameCompleteYouWinByChallenge",
  GameCompleteYouWinByTimeout = "GameCompleteYouWinByTimeout",
  TransferFunds = "TransferFunds",
}

export enum GameStatePlayer2 {
  ChooseGame = "ChooseGame",
  ChooseNetwork = "SetupNetwork",
  ChooseRole = "ChooseRole",
  // SetupFunding = "SetupFunding", This is only in testnet
  SetupConnection = "SetupConnection",
  SetupProgram = "SetupProgram",
  WaitingStartGame = "WaitingStartGame",
  AnswerGame = "AnswerGame",
  WaitingAnswer = "WaitingAnswer",
  GameCompleteYouLoseByChallenge = "GameCompleteYouLoseByChallenge",
  GameCompleteYouLoseByTimeout = "GameCompleteYouLoseByTimeout",
  GameCompleteYouLoseByAccept = "GameCompleteYouLoseByAccept",
  GameCompleteYouWinByChallenge = "GameCompleteYouWinByChallenge",
  GameCompleteYouWinByTimeout = "GameCompleteYouWinByTimeout",
  TransferFunds = "TransferFunds",
}

export interface GameNumbersToAdd {
  number1?: number;
  number2?: number;
}
