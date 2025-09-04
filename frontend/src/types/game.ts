export enum GameState {
  ChooseGame = "ChooseGame",
  ChooseNetwork = "SetupNetwork",
  ChooseRole = "ChooseRole",
  // SetupFunding = "SetupFunding", This is only in testnet
  SetupConnection = "SetupConnection",
  SetupProgram = "SetupProgram",
  StartGame = "StartGame",
  WaitingAnswer = "WaitingAnswer",
  ChooseAction = "ChooseAction",
  ChallengeAnswer = "ChallengeAnswer",
  WaitingStartGame = "WaitingStartGame",
  AnswerGame = "AnswerGame",
  GameCompleteYouLoseByChallenge = "GameCompleteYouLoseByChallenge",
  GameCompleteYouLoseByTimeout = "GameCompleteYouLoseByTimeout",
  GameCompleteYouLoseByAccept = "GameCompleteYouLoseByAccept",
  GameCompleteYouWinByChallenge = "GameCompleteYouWinByChallenge",
  GameCompleteYouWinByTimeout = "GameCompleteYouWinByTimeout",
  GameCompleteYouWinByAccept = "GameCompleteYouWinByAccept",
  TransferFunds = "TransferFunds",
}

export interface GameNumbersToAdd {
  number1?: number;
  number2?: number;
}

export enum PlayerRole {
  Player1 = "Player1",
  Player2 = "Player2",
}

export type Player = PlayerRole.Player1 | PlayerRole.Player2;
