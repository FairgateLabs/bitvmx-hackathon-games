"use client";

import { TimeRemaining } from "@/components/ui/time-remaining";
import { ChallengeWinGame } from "@/components/player1/challege-win-game";
import { ChallengeLoseGame } from "@/components/player1/challenge-lose-game";
import { AcceptLoseGame } from "@/components/player1/accept-lose-game";
import { TimeoutWinGame } from "@/components/player1/timeout-win-game";

export default function TransactionsPage() {
  return (
    <>
      <ChallengeWinGame />
      <ChallengeLoseGame />
      <AcceptLoseGame />
      <TimeoutWinGame />
    </>
  );
}
