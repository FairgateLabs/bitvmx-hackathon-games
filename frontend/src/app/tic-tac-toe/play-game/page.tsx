"use client";

import {
  Card,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { ChooseRole } from "@/components/common/game-role-selector";
import { ChooseNetwork } from "@/components/common/choose-network";
import { EnumPlayerRole } from "@/types/game";
import { useCurrentGame } from "@/hooks/useGame";
import { useState } from "react";
import { useNetworkQuery } from "@/hooks/useNetwork";

export default function TicTacToePage() {
  const { data: network } = useNetworkQuery();
  const [role, setRole] = useState<EnumPlayerRole | null>(null);

  const { data: currentGame } = useCurrentGame();

  if (!currentGame && !network) {
    return <ChooseNetwork />;
  }

  if (!currentGame && !role) {
    return (
      <ChooseRole
        title="⭕ Tic Tac Toe Game"
        description="Choose the role you want to play"
        subtitle="Two players compete by playing Tic Tac Toe. Who are you?"
        onSelectRole={setRole}
      />
    );
  }

  return (
    <div className="container mx-auto p-6 max-w-4xl">
      <Card>
        <CardHeader>
          <CardTitle className="text-2xl">
            {role === EnumPlayerRole.Player1
              ? "➕ Player 1 - Tic Tac Toe"
              : "🤝 Player 2 - Tic Tac Toe"}
          </CardTitle>
          <CardDescription>
            {role === EnumPlayerRole.Player1
              ? "Create the game and choose the numbers to add"
              : "Join the game and answer the sum"}
          </CardDescription>
        </CardHeader>

        {/* <CardContent className="space-y-6">
          <NetworkInfo expanded={false} />
          <WalletSection expanded={false} />
          <UtxoExchange />
          {role === EnumPlayerRole.Player1 && (
            <>
              {gameStatus === "SetupParticipants" && <SetupGamePlayer1 />}
              {gameStatus === "StartGame" && <SetupGamePlayer1 />}
              {gameStatus === "BindNumbersToProgram" && <SetupGamePlayer1 />}
              {gameStatus === "SubmitGameData" && <StartGame />}
              {gameStatus === "SubmitSum" && <WaitingAnswer />}
              {gameStatus === "ProgramDecision" && <ChooseAction />}
              {gameStatus?.GameComplete &&
                (gameStatus.GameComplete.outcome === "Lose" ? (
                  <AcceptLoseGame />
                ) : gameStatus.GameComplete.outcome === "Win" ? (
                  <div>Game Complete - You Win!</div>
                ) : (
                  <div>Game Complete - Draw!</div>
                ))}
              {gameStatus === "TransferFunds" && (
                <div>Transferring funds...</div>
              )}
              {gameStatus === "Finished" && <div>Game Finished - You Win!</div>}
            </>
          )}

          {role === EnumPlayerRole.Player2 && (
            <>
              {gameStatus === "SetupParticipants" && <SetupGamePlayer2 />}
              {gameStatus === "CreateProgram" && <WaitingStartGame />}
              {gameStatus === "BindNumbersToProgram" && <WaitingStartGame />}
              {gameStatus === "WaitForSum" && <SubmitGameData />}
              {gameStatus === "SubmitSum" && <WaitingForAnswer />}
              {gameStatus === "ProgramDecision" && <WaitingForAnswer />}
              {gameStatus === "Challenge" && <ChallengeAnswer />}
              {gameStatus?.GameComplete &&
                (gameStatus.GameComplete.outcome === "Win" ? (
                  <AcceptWinGame />
                ) : gameStatus.GameComplete.outcome === "Lose" ? (
                  <div>Game Complete - You Lose!</div>
                ) : (
                  <div>Game Complete - Draw!</div>
                ))}
              {gameStatus === "TransferBetFunds" && (
                <div>Transferring funds...</div>
              )}
              {gameStatus === "Finished" && (
                <div>Game Finished - You Lose!</div>
              )}
            </>
          )}
        </CardContent> */}
      </Card>
    </div>
  );
}
