"use client";

import { useState } from "react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { ChooseRole, GameRole } from "@/components/common/game-role-selector";
import { WalletSection } from "@/components/common/wallet-section";
import { SetupGame as SetupGamePlayer1 } from "@/components/player1/setup-game";
import { SetupGame as SetupGamePlayer2 } from "@/components/player2/setup-game";
import { ChooseAction } from "@/components/player1/choose-actions";
import { NetworkInfo } from "@/components/common/network-info";
import { PeerConnectionInfo } from "@/components/common/peer-connection-info";
import { PeerConnectionInput } from "@/components/common/peer-connection-input";
import { ChooseNetwork } from "@/components/common/choose-network";
import { GameState } from "@/types/gameState";
import { StartGame } from "@/components/player1/start-game";
import { useGameState } from "@/hooks/useGameState";
import { useGameRole } from "@/hooks/useGameRole";
import { AcceptLoseGame } from "@/components/player1/accept-lose-game";
import { YouWin } from "@/components/player1/challege-win-game";
import { ChallengeAnswer } from "@/components/player1/challenge-answer";
import { AnswerGame } from "@/components/player2/answer-game";
import { WaitingForAnswer } from "@/components/player2/waiting-for-answer";

export default function AddNumbersPage() {
  const { data: gameState } = useGameState();
  const { data: role } = useGameRole();

  if (gameState === GameState.SetupNetwork) {
    return <ChooseNetwork />;
  }

  if (gameState === GameState.ChooseRole) {
    return <ChooseRole />;
  }

  return (
    <div className="container mx-auto p-6 max-w-4xl">
      <Card>
        <CardHeader>
          <CardTitle className="text-2xl">
            {role === GameRole.Player1
              ? "➕ Player 1 - Add Numbers"
              : "🤝 Player 2 - Add Numbers"}
          </CardTitle>
          <CardDescription>
            {role === GameRole.Player1
              ? "Create the game and choose the numbers to add"
              : "Join the game and answer the sum"}
          </CardDescription>
        </CardHeader>

        <CardContent className="space-y-6">
          <NetworkInfo />
          <WalletSection />
          <PeerConnectionInfo />
          <PeerConnectionInput />
          {role === GameRole.Player1 && (
            <>
              {gameState === GameState.SetupProgram && <SetupGamePlayer1 />}
              {gameState === GameState.StartGame && <StartGame />}
              {gameState === GameState.ChooseAction && <ChooseAction />}
            </>
          )}

          {role === GameRole.Player2 && (
            <>
              {gameState === GameState.SetupProgram && <SetupGamePlayer2 />}
              {gameState === GameState.StartGame && <AnswerGame />}
              {gameState === GameState.ChooseAction && <WaitingForAnswer />}
            </>
          )}

          {gameState === GameState.ChallengeAnswer && <ChallengeAnswer />}
          {gameState === GameState.GameCompleteYouLose && <AcceptLoseGame />}
          {gameState === GameState.GameCompleteYouWin && <YouWin />}
        </CardContent>
      </Card>
    </div>
  );
}
