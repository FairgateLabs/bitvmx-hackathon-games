"use client";

// import { useEffect } from "react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { ChooseRole } from "@/components/common/game-role-selector";
import { WalletSection } from "@/components/common/wallet-section";
import { SetupGame as SetupGamePlayer1 } from "@/components/tic-tac-toe/player1/setup-game";
import { SetupGame as SetupGamePlayer2 } from "@/components/tic-tac-toe/player2/setup-game";
import { ChooseAction } from "@/components/player1/choose-actions";
import { NetworkInfo } from "@/components/common/network-info";
import { PeerConnectionInfo } from "@/components/common/peer-connection-info";
import { PeerConnectionInput } from "@/components/common/peer-connection-input";
import { UtxoExchange } from "@/components/common/utxo-exchange";
import { ChooseNetwork } from "@/components/common/choose-network";
import { EnumPlayerRole } from "@/types/game";
// import { AddNumbersGameStatus } from "../../../../../backend/bindings/AddNumbersGameStatus";
import { StartGame } from "@/components/tic-tac-toe/common/start-game";
// import { useGameState, useNextGameState } from "@/hooks/useGameState";
import { AcceptLoseGame } from "@/components/player1/accept-lose-game";
// import { ChallengeWinGame } from "@/components/player1/challege-win-game";
// import { ChallengeWinGame as ChallengeWinGamePlayer2 } from "@/components/player2/challenge-win-game";
import { ChallengeAnswer } from "@/components/player1/challenge-answer";
import { AnswerGame } from "@/components/player2/answer-game";
import { WaitingForAnswer } from "@/components/player2/waiting-for-answer";
// import { TimeoutWinGame } from "@/components/player1/timeout-win-game";
import { WaitingAnswer } from "@/components/player1/waiting-answer";
// import { ChallengeLoseGame } from "@/components/player1/challenge-lose-game";
// import { TimeoutLoseGame } from "@/components/player2/timeout-lose-game";
import { AcceptWinGame } from "@/components/player2/accept-win-game";
import { WaitingStartGame } from "@/components/player2/waiting-start-game";
import { useCurrentGame } from "@/hooks/useGame";
import { useState } from "react";
import { useNetworkQuery } from "@/hooks/useNetwork";

export default function TicTacToePage() {
  // const { data: gameStatus } = useGameState();
  const { data: network } = useNetworkQuery();
  const [role, setRole] = useState<EnumPlayerRole | null>(null);

  // const { mutate: nextGameState } = useNextGameState();
  const { data: currentGame, isLoading: isGameLoading } = useCurrentGame();
  const gameStatus = currentGame?.status;

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

        <CardContent className="space-y-6">
          <NetworkInfo />
          <WalletSection />
          {/* <PeerConnectionInfo aggregatedId={} />
          <PeerConnectionInput aggregatedId={currentGame?.bitvmx_program_properties.aggregated_id || ""} /> */}
          <UtxoExchange />
          {role === EnumPlayerRole.Player1 && (
            <>
              {gameStatus === "SetupParticipants" && <SetupGamePlayer1 />}
              {gameStatus === "CreateProgram" && <SetupGamePlayer1 />}
              {gameStatus === "BindNumbersToProgram" && <SetupGamePlayer1 />}
              {gameStatus === "WaitForSum" && <StartGame />}
              {gameStatus === "SubmitSum" && <WaitingAnswer />}
              {gameStatus === "ProgramDecision" && <ChooseAction />}
              {gameStatus === "Challenge" && <ChallengeAnswer />}
              {typeof gameStatus === "object" &&
                "GameComplete" in gameStatus &&
                (gameStatus.GameComplete.outcome === "Lose" ? (
                  <AcceptLoseGame />
                ) : gameStatus.GameComplete.outcome === "Win" ? (
                  <div>Game Complete - You Win!</div>
                ) : (
                  <div>Game Complete - Draw!</div>
                ))}
              {gameStatus === "TransferBetFunds" && (
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
              {gameStatus === "WaitForSum" && <AnswerGame />}
              {gameStatus === "SubmitSum" && <WaitingForAnswer />}
              {gameStatus === "ProgramDecision" && <WaitingForAnswer />}
              {gameStatus === "Challenge" && <ChallengeAnswer />}
              {typeof gameStatus === "object" &&
                "GameComplete" in gameStatus &&
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
        </CardContent>
      </Card>
    </div>
  );
}
