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
import { NetworkInfo } from "@/components/common/network-info";
import { SetupParticipantInfo } from "@/components/common/setup-participant-info";
import { SetupParticipantInput } from "@/components/common/setup-participant-input";
import { FundingExchange } from "@/components/common/funding-exchange";
import { ChooseNetwork } from "@/components/common/choose-network";
import { SubmitGameData } from "@/components/player2/submit-game-data";
import { useCurrentGame } from "@/hooks/useGame";
import { useNetworkQuery } from "@/hooks/useNetwork";
import { BackendStatus } from "@/components/common/backend-status";
import { AggregatedKey } from "@/components/common/aggregated-key";
import { useEffect, useState } from "react";
import { PlayerRole } from "../../../../../backend/bindings/PlayerRole";
import { EnumPlayerRole } from "@/types/game";
import { PlaceBet } from "@/components/common/place-bet";
import { SetupGame } from "@/components/common/setup-game";
import { StartGame } from "@/components/player1/start-game";
import { WaitingAnswer } from "@/components/player1/waiting-answer";
import { WaitingStartGame } from "@/components/player2/waiting-start-game";
import { GameOutcome } from "@/components/common/game-outcome";

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

  let isGameComplete =
    typeof game?.status === "object" && "GameComplete" in game?.status;

  let isSetupFunding =
    game?.status === "SetupFunding" ||
    (game?.status === "SetupGame" && role === EnumPlayerRole.Player1);

  return (
    <BackendStatus>
      {isGameLoading && game !== undefined ? (
        <div className="container mx-auto p-6 max-w-4xl">
          <div className="text-center">
            <p className="text-lg">⏳ Loading game...</p>
          </div>
        </div>
      ) : !game?.program_id && !network ? (
        <ChooseNetwork />
      ) : !game?.program_id && !role ? (
        <ChooseRole
          title="🎮 Add Numbers Game"
          description="Choose the role you want to play"
          subtitle="Two players compete by adding numbers. Who are you?"
          onSelectRole={setRole}
        />
      ) : (
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
              {!game && (
                <SetupParticipantInput
                  aggregatedId={aggregatedId}
                  role={role!}
                />
              )}

              {game?.bitvmx_program_properties.aggregated_key && (
                <AggregatedKey expand={game?.status === "PlaceBet"} />
              )}

              {game && game.status === "PlaceBet" && <PlaceBet />}

              {isSetupFunding && <FundingExchange expand={isSetupFunding} />}

              {game?.status === "SetupGame" && <SetupGame />}

              {role === EnumPlayerRole.Player1 && (
                <>
                  {game?.status === "StartGame" && <StartGame />}
                  {game?.status === "SubmitGameData" && <WaitingAnswer />}
                </>
              )}

              {role === EnumPlayerRole.Player2 && (
                <>
                  {game?.status === "StartGame" && <WaitingStartGame />}
                  {game?.status === "StartGame" && <SubmitGameData />}
                </>
              )}

              {isGameComplete && <GameOutcome />}
            </CardContent>
          </Card>
        </div>
      )}
    </BackendStatus>
  );
}
