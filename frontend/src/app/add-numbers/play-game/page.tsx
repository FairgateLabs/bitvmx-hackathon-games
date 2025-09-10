"use client";

// import { useEffect, useState } from "react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { ChooseRole } from "@/components/common/game-role-selector";
import { WalletSection } from "@/components/common/wallet-section";
import { SetupGame } from "@/components/common/setup-game";
import { ChooseAction } from "@/components/player1/choose-actions";
import { NetworkInfo } from "@/components/common/network-info";
import { PeerConnectionInfo } from "@/components/common/peer-connection-info";
import { PeerConnectionInput } from "@/components/common/peer-connection-input";
import { UtxoExchange } from "@/components/common/utxo-exchange";
import { ChooseNetwork } from "@/components/common/choose-network";
import { PlayerRole } from "@/types/game";
// import { AddNumbersGameStatus } from "../../../../../backend/bindings/AddNumbersGameStatus";
import { StartGame } from "@/components/player1/start-game";
// import { useGameState, useNextGameState } from "@/hooks/useGameState";
import { useGameRole } from "@/hooks/useGameRole";
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
import { useNetworkQuery } from "@/hooks/useNetwork";
import { AggregatedKey } from "@/components/common/aggregated-key";
import { useEffect, useState } from "react";

export default function AddNumbersPage() {
  const { data: role } = useGameRole();
  const { data: network } = useNetworkQuery();
  const [aggregatedId, setAggregatedId] = useState<string>("");
  const { data: currentGame, isLoading: isGameLoading } = useCurrentGame();

  useEffect(() => {
    if (!role || !!currentGame?.program_id) return;
    if (!currentGame?.program_id) {
      if (role === PlayerRole.Player1) {
        const aggregatedId = crypto.randomUUID();
        setAggregatedId(aggregatedId);
      } else {
        setAggregatedId("");
      }
    } else {
      setAggregatedId(
        currentGame?.bitvmx_program_properties.aggregated_id ?? ""
      );
    }
  }, [currentGame, role]);

  if (isGameLoading) {
    return (
      <div className="container mx-auto p-6 max-w-4xl">
        <div className="text-center">
          <p className="text-lg">Loading game...</p>
        </div>
      </div>
    );
  }

  if (!currentGame?.program_id && !network) {
    return <ChooseNetwork />;
  }

  if (!currentGame?.program_id && !role) {
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
          <PeerConnectionInfo aggregatedId={aggregatedId} />
          <PeerConnectionInput aggregatedId={aggregatedId} />
          <UtxoExchange />
          {currentGame?.status === "CreateProgram" && <SetupGame />}
          <AggregatedKey />

          {role === PlayerRole.Player1 && (
            <>
              {currentGame?.status === "WaitForSum" && <StartGame />}
              {currentGame?.status === "SubmitSum" && <WaitingAnswer />}
              {currentGame?.status === "ProgramDecision" && <ChooseAction />}
              {currentGame?.status === "Challenge" && <ChallengeAnswer />}
              {typeof currentGame?.status === "object" &&
                "GameComplete" in currentGame?.status &&
                (currentGame?.status.GameComplete.outcome === "Lose" ? (
                  <AcceptLoseGame />
                ) : currentGame?.status.GameComplete.outcome === "Win" ? (
                  <div>Game Complete - You Win!</div>
                ) : (
                  <div>Game Complete - Draw!</div>
                ))}
              {currentGame?.status === "TransferBetFunds" && (
                <div>Transferring funds...</div>
              )}
              {currentGame?.status === "Finished" && (
                <div>Game Finished - You Win!</div>
              )}
            </>
          )}

          {role === PlayerRole.Player2 && (
            <>
              {currentGame?.status === "WaitForSum" && <AnswerGame />}
              {currentGame?.status === "SubmitSum" && <WaitingForAnswer />}
              {currentGame?.status === "ProgramDecision" && (
                <WaitingForAnswer />
              )}
              {currentGame?.status === "Challenge" && <ChallengeAnswer />}
              {typeof currentGame?.status === "object" &&
                "GameComplete" in currentGame?.status &&
                (currentGame?.status.GameComplete.outcome === "Win" ? (
                  <AcceptWinGame />
                ) : currentGame?.status.GameComplete.outcome === "Lose" ? (
                  <div>Game Complete - You Lose!</div>
                ) : (
                  <div>Game Complete - Draw!</div>
                ))}
              {currentGame?.status === "TransferBetFunds" && (
                <div>Transferring funds...</div>
              )}
              {currentGame?.status === "Finished" && (
                <div>Game Finished - You Lose!</div>
              )}
            </>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
