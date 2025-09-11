"use client";

import {
  GameEndResult,
  TicTacToeBoard,
} from "@/components/tic-tac-toe/common/tic-tac-toe-board";
import { ChooseAction } from "@/components/tic-tac-toe/common/choose-actions";
import { useState } from "react";

export default function TransactionsPage() {
  const [gameResult, setGameResult] = useState<GameEndResult | null>(null);
  return (
    <>
      <TicTacToeBoard
        onGameEnd={(gameResult) => {
          setGameResult(gameResult);
          console.log(gameResult);
        }}
      />
      {gameResult && (
        <ChooseAction
          winner={gameResult.winner}
          isTimeout={gameResult.isTimeout}
        />
      )}

      {/* <ChallengeWinGame />
      <ChallengeLoseGame />
      <AcceptLoseGame />
      <TimeoutWinGame />
      <WaitingAnswer />
      ----- Player 2 -----
      <WaitingStartGame />
      <ChallengeWinGame2 />
      <ChallengeLoseGame2 />
      <AcceptWinGame2 />
      <TimeoutWinGame2 />
       */}
    </>
  );
}
