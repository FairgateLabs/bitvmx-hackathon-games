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

export default function TransactionsPage() {
  return (
    <>
      <ChallengeWinGame />
      <ChallengeLoseGame />
      <AcceptLoseGame />
      <TimeoutWinGame />
      ----- Player 2 -----
      <ChallengeWinGame2 />
      <ChallengeLoseGame2 />
      <AcceptWinGame2 />
      <TimeoutWinGame2 />
    </>
  );
}
