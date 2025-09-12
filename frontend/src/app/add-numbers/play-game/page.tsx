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
// import { AddNumbersGameStatus } from "../../../../../backend/bindings/AddNumbersGameStatus";
import { StartGame } from "@/components/player1/start-game";
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
import { useCurrentGame } from "@/hooks/useGame";
import { useNetworkQuery } from "@/hooks/useNetwork";
import { AggregatedKey } from "@/components/common/aggregated-key";
import { useEffect, useState } from "react";
import { PlayerRole } from "../../../../../backend/bindings/PlayerRole";
import { EnumPlayerRole } from "@/types/game";
import { BettingInfo } from "@/components/common/betting-info";

export default function AddNumbersPage() {
  const { data: network } = useNetworkQuery();
  const [aggregatedId, setAggregatedId] = useState<string>("");
  const { data: game, isLoading: isGameLoading } = useCurrentGame();
  const [role, setRole] = useState<PlayerRole | null>(null);

  useEffect(() => {
    if (!game) {
      if (role === EnumPlayerRole.Player1) {
        const aggregatedId = crypto.randomUUID();
        setAggregatedId(aggregatedId);
      } else {
        setAggregatedId("");
      }
    } else {
      setAggregatedId(game?.bitvmx_program_properties.aggregated_id ?? "");
      setRole(game?.role);
    }
  }, [game, role]);

  if (isGameLoading && game !== undefined) {
    return (
      <div className="container mx-auto p-6 max-w-4xl">
        <div className="text-center">
          <p className="text-lg">⏳ Loading game...</p>
        </div>
      </div>
    );
  }

  if (!game?.program_id && !network) {
    return <ChooseNetwork />;
  }

  if (!game?.program_id && !role) {
    return (
      <ChooseRole
        title="🎮 Add Numbers Game"
        description="Choose the role you want to play"
        subtitle="Two players compete by adding numbers. Who are you?"
        onSelectRole={setRole}
      />
    );
  }

  {
    console.log("game", !game, game?.status === "SetupParticipants");
  }
  return (
    <div className="container mx-auto p-6 max-w-4xl">
      <Card>
        <CardHeader>
          <CardTitle className="text-2xl">
            {role === EnumPlayerRole.Player1
              ? "➕ Player 1 - Add Numbers Game"
              : "🤝 Player 2 - Add Numbers Game"}
          </CardTitle>
          <CardDescription>
            {role === EnumPlayerRole.Player1
              ? "Create the game and choose the numbers to add"
              : "Join the game and answer the sum"}
          </CardDescription>
        </CardHeader>

        <CardContent className="space-y-6">
          <NetworkInfo
            expanded={!game || game?.status === "SetupParticipants"}
          />
          <WalletSection
            expanded={!game || game?.status === "SetupParticipants"}
          />
          <PeerConnectionInfo
            aggregatedId={aggregatedId}
            role={role!}
            expanded={!game || game?.status === "SetupParticipants"}
          />
          <PeerConnectionInput
            aggregatedId={aggregatedId}
            role={role!}
            expanded={!game || game?.status === "SetupParticipants"}
          />
          {game && game.status !== "PlaceBet" && <AggregatedKey />}
          {game && game.status !== "SetupParticipants" && <BettingInfo />}
          {game && game.status !== "SetupFunding" && <UtxoExchange />}
          {game && game?.status === "CreateProgram" && <SetupGame />}

          {role === EnumPlayerRole.Player1 && (
            <>
              {game?.status === "WaitForSum" && <StartGame />}
              {game?.status === "SubmitSum" && <WaitingAnswer />}
              {game?.status === "ProgramDecision" && <ChooseAction />}
              {game?.status === "Challenge" && <ChallengeAnswer />}
              {typeof game?.status === "object" &&
                "GameComplete" in game?.status &&
                (game?.status.GameComplete.outcome === "Lose" ? (
                  <AcceptLoseGame />
                ) : game?.status.GameComplete.outcome === "Win" ? (
                  <div>Game Complete - You Win!</div>
                ) : (
                  <div>Game Complete - Draw!</div>
                ))}
              {game?.status === "TransferBetFunds" && (
                <div>Transferring funds...</div>
              )}
              {game?.status === "Finished" && (
                <div>Game Finished - You Win!</div>
              )}
            </>
          )}

          {role === EnumPlayerRole.Player2 && (
            <>
              {game?.status === "WaitForSum" && <AnswerGame />}
              {game?.status === "SubmitSum" && <WaitingForAnswer />}
              {game?.status === "ProgramDecision" && <WaitingForAnswer />}
              {game?.status === "Challenge" && <ChallengeAnswer />}
              {typeof game?.status === "object" &&
                "GameComplete" in game?.status &&
                (game?.status.GameComplete.outcome === "Win" ? (
                  <AcceptWinGame />
                ) : game?.status.GameComplete.outcome === "Lose" ? (
                  <div>Game Complete - You Lose!</div>
                ) : (
                  <div>Game Complete - Draw!</div>
                ))}
              {game?.status === "TransferBetFunds" && (
                <div>Transferring funds...</div>
              )}
              {game?.status === "Finished" && (
                <div>Game Finished - You Lose!</div>
              )}
            </>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
