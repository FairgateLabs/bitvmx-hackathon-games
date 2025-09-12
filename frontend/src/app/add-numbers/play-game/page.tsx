"use client";

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
import { NetworkInfo } from "@/components/common/network-info";
import { SetupParticipantInfo } from "@/components/common/setup-participant-info";
import { SetupParticipantInput } from "@/components/common/setup-participant-input";
import { UtxoExchange } from "@/components/common/utxo-exchange";
import { ChooseNetwork } from "@/components/common/choose-network";
import { StartGame } from "@/components/player1/start-game";
import { AcceptLoseGame } from "@/components/player1/accept-lose-game";
import { SubmitGameData } from "@/components/player2/submit-game-data";
import { AcceptWinGame } from "@/components/player2/accept-win-game";
import { useCurrentGame } from "@/hooks/useGame";
import { useNetworkQuery } from "@/hooks/useNetwork";
import { AggregatedKey } from "@/components/common/aggregated-key";
import { useEffect, useState } from "react";
import { PlayerRole } from "../../../../../backend/bindings/PlayerRole";
import { EnumPlayerRole } from "@/types/game";
import { PlaceBet } from "@/components/common/place-bet";

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
          <SetupParticipantInfo
            aggregatedId={aggregatedId}
            role={role!}
            expanded={!game || game?.status === "SetupParticipants"}
          />
          {game && game.status === "SetupParticipants" && (
            <SetupParticipantInput aggregatedId={aggregatedId} role={role!} />
          )}
          {game && game.status !== "PlaceBet" && <AggregatedKey />}
          {game && game.status === "PlaceBet" && <PlaceBet />}
          {game && game.status === "SetupFunding" && <UtxoExchange />}
          {game && game?.status === "StartGame" && <SetupGame />}
          {game?.status === "StartGame" && <StartGame />}

          {role === EnumPlayerRole.Player1 && (
            <>
              {typeof game?.status === "object" &&
                "GameComplete" in game?.status &&
                (game?.status.GameComplete.outcome === "Lose" ? (
                  <AcceptLoseGame />
                ) : game?.status.GameComplete.outcome === "Win" ? (
                  <div>Game Complete - You Win!</div>
                ) : (
                  <div>Game Complete - Draw!</div>
                ))}
              {/* {game?.status === "TransferFunds" && (
                <div>Transferring funds...</div>
              )} */}
              {game?.status === "Finished" && (
                <div>Game Finished - You Win!</div>
              )}
            </>
          )}

          {role === EnumPlayerRole.Player2 && (
            <>
              {game?.status === "SubmitGameData" && <SubmitGameData />}
              {typeof game?.status === "object" &&
                "GameComplete" in game?.status &&
                (game?.status.GameComplete.outcome === "Win" ? (
                  <AcceptWinGame />
                ) : game?.status.GameComplete.outcome === "Lose" ? (
                  <div>Game Complete - You Lose!</div>
                ) : (
                  <div>Game Complete - Draw!</div>
                ))}

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
