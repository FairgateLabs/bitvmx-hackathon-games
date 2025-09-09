"use client";

import { useEffect, useState } from "react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { ChooseRole } from "@/components/common/game-role-selector";
import { WalletSection } from "@/components/common/wallet-section";
import { SetupGame as SetupGamePlayer1 } from "@/components/player1/setup-game";
import { SetupGame as SetupGamePlayer2 } from "@/components/player2/setup-game";
import { ChooseAction } from "@/components/player1/choose-actions";
import { NetworkInfo } from "@/components/common/network-info";
import { PeerConnectionInfo } from "@/components/common/peer-connection-info";
import { PeerConnectionInput } from "@/components/common/peer-connection-input";
import { ChooseNetwork } from "@/components/common/choose-network";
import { GameState, PlayerRole } from "@/types/game";
import { StartGame } from "@/components/player1/start-game";
import { useGameState, useNextGameState } from "@/hooks/useGameState";
import { useGameRole } from "@/hooks/useGameRole";
import { AcceptLoseGame } from "@/components/player1/accept-lose-game";
import { ChallengeWinGame } from "@/components/player1/challege-win-game";
import { ChallengeWinGame as ChallengeWinGamePlayer2 } from "@/components/player2/challenge-win-game";
import { ChallengeAnswer } from "@/components/player1/challenge-answer";
import { AnswerGame } from "@/components/player2/answer-game";
import { WaitingForAnswer } from "@/components/player2/waiting-for-answer";
import { TimeoutWinGame } from "@/components/player1/timeout-win-game";
import { WaitingAnswer } from "@/components/player1/waiting-answer";
import { ChallengeLoseGame } from "@/components/player1/challenge-lose-game";
import { TimeoutLoseGame } from "@/components/player2/timeout-lose-game";
import { AcceptWinGame } from "@/components/player2/accept-win-game";
import { WaitingStartGame } from "@/components/player2/waiting-start-game";
import { useCurrentGame } from "@/hooks/useGame";

export default function AddNumbersPage() {
  const { data: gameState } = useGameState();
  const { data: role } = useGameRole();
  const { mutate: nextGameState } = useNextGameState();
  const { data: currentGame } = useCurrentGame();

  useEffect(() => {
    if (gameState === GameState.ChooseGame) {
      nextGameState(GameState.ChooseRole);
    }
  });

  if (!currentGame && gameState === GameState.ChooseNetwork) {
    return <ChooseNetwork />;
  }

  if (!currentGame && gameState === GameState.ChooseRole) {
    return (
      <ChooseRole
        title="🎮 Add Numbers Game"
        description="Choose the role you want to play"
        subtitle="Two players compete by adding numbers. Who are you?"
      />
    );
  }

  return (
    <div className="container mx-auto p-6 max-w-4xl">
      <Card>
        <CardHeader>
          <CardTitle className="text-2xl">
            {role === PlayerRole.Player1
              ? "➕ Player 1 - Add Numbers"
              : "🤝 Player 2 - Add Numbers"}
          </CardTitle>
          <CardDescription>
            {role === PlayerRole.Player1
              ? "Create the game and choose the numbers to add"
              : "Join the game and answer the sum"}
          </CardDescription>
        </CardHeader>

        <CardContent className="space-y-6">
          <NetworkInfo />
          <WalletSection />
          <PeerConnectionInfo />
          <PeerConnectionInput />
          {role === PlayerRole.Player1 && (
            <>
              {gameState === GameState.SetupProgram && <SetupGamePlayer1 />}
              {gameState === GameState.StartGame && <StartGame />}
              {gameState === GameState.WaitingAnswer && <WaitingAnswer />}
              {gameState === GameState.ChooseAction && <ChooseAction />}

              {gameState === GameState.ChallengeAnswer && <ChallengeAnswer />}

              {gameState === GameState.GameCompleteYouLoseByAccept && (
                <AcceptLoseGame />
              )}

              {gameState === GameState.GameCompleteYouWinByTimeout && (
                <TimeoutWinGame />
              )}

              {gameState === GameState.GameCompleteYouLoseByChallenge && (
                <ChallengeLoseGame />
              )}
              {gameState === GameState.GameCompleteYouLoseByTimeout && (
                <TimeoutLoseGame />
              )}

              {gameState === GameState.GameCompleteYouWinByChallenge && (
                <ChallengeWinGame />
              )}
            </>
          )}

          {role === PlayerRole.Player2 && (
            <>
              {gameState === GameState.SetupProgram && <SetupGamePlayer2 />}
              {gameState === GameState.WaitingStartGame && <WaitingStartGame />}
              {gameState === GameState.StartGame && <AnswerGame />}
              {gameState === GameState.ChooseAction && <WaitingForAnswer />}
              {gameState === GameState.ChallengeAnswer && <ChallengeAnswer />}

              {gameState === GameState.GameCompleteYouLoseByChallenge && (
                <ChallengeLoseGame />
              )}
              {gameState === GameState.GameCompleteYouLoseByTimeout && (
                <TimeoutLoseGame />
              )}

              {gameState === GameState.GameCompleteYouWinByChallenge && (
                <ChallengeWinGamePlayer2 />
              )}

              {gameState === GameState.GameCompleteYouWinByAccept && (
                <AcceptWinGame />
              )}
              {gameState === GameState.GameCompleteYouWinByTimeout && (
                <TimeoutWinGame />
              )}
            </>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
