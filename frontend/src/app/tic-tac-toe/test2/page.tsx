"use client";

import { TimeRemaining } from "@/components/ui/time-remaining";
import { ChallengeWinGame } from "@/components/player1/challege-win-game";
import { ChallengeLoseGame } from "@/components/player1/challenge-lose-game";
import { AcceptLoseGame } from "@/components/player1/accept-lose-game";
import { TimeoutWinGame } from "@/components/player1/timeout-win-game";
import { ChallengeWinGame as ChallengeWinGame2 } from "@/components/player2/challenge-win-game";
import { ChallengeLoseGame as ChallengeLoseGame2 } from "@/components/player2/challenge-lose-game";
import { AcceptWinGame as AcceptWinGame2 } from "@/components/player2/accept-win-game";
import { TimeoutLoseGame as TimeoutWinGame2 } from "@/components/player2/timeout-lose-game";
import { WaitingAnswer } from "@/components/player1/waiting-answer";
import { WaitingStartGame } from "@/components/player2/waiting-start-game";
import { Titan_One } from "next/font/google";
import { TicTacToeBoard } from "@/components/tic-tac-toe/common/tic-tac-toe-board";

export default function TransactionsPage() {
  return (
    <>
      <TicTacToeBoard onGameEnd={() => {}} />

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
