"use client";

import { useState } from "react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { GameRoleSelector, GameRole } from "@/components/game-role-selector";
import { WalletSection } from "@/components/wallet-section";
import { Player1GameSetup } from "@/components/player1-game-setup";
import { Player2GameSetup } from "@/components/player2-game-setup";
import { GameActions } from "@/components/game-actions";
import { NetworkInfo } from "@/components/network-info";
import { PeerConnectionInfo } from "@/components/peer-connection-info";
import { PeerConnectionInput } from "@/components/peer-connection-input";
import { GameUUIDInput } from "@/components/game-uuid-input";
import { Button } from "@/components/ui/button";
import { NetworkType } from "@/types/network";
import { GameState } from "@/types/gameState";
import { useGameState } from "@/hooks/useGameState";
import { GameStartNotification } from "@/components/game-start-notification";

export default function AddNumbersPage() {
  const [gameRole, setGameRole] = useState<GameRole | null>(null);
  const [gameId, setGameId] = useState("");
  const [peerIP, setPeerIP] = useState("");
  const [peerPort, setPeerPort] = useState("");
  const [isGameStarted, setIsGameStarted] = useState(false);
  const { data: gameState } = useGameState();

  const [networkSelected, setNetworkSelected] = useState<NetworkType | null>(
    null
  );

  if (!networkSelected) {
    return (
      <div className="container mx-auto p-6 max-w-[840px]">
        <Card>
          <CardHeader>
            <CardTitle className="text-2xl text-center">
              🔽 Select Network
            </CardTitle>
            <CardDescription className="text-center">
              Choose the Bitcoin network for your game session.
              <br />
              In Regtest, you have the flexibility to simulate transactions
              without real funds, ideal for testing and development. <br />
              In Testnet, transactions mimic real-world scenarios, requiring you
              to fund your wallet with testnet Bitcoins.
            </CardDescription>
          </CardHeader>
          <CardContent className="grid grid-cols-2 gap-4">
            <Button
              onClick={() => setNetworkSelected(NetworkType.Regtest)}
              className="flex text-center h-40 text-lg cursor-pointer"
              variant="outline"
            >
              <div className="flex flex-col w-full gap-2 h-[130px]">
                🔗 Regtest
                <p className="text-sm font-normal max-w-xs whitespace-normal block">
                  Easy to use and ideal for testing and development. Funds are
                  given to you automatically without worrying about funding your
                  wallet.
                </p>
              </div>
            </Button>
            <Button
              onClick={() => setNetworkSelected(NetworkType.Testnet)}
              className="flex text-center h-40 text-lg cursor-pointer"
              variant="outline"
              disabled
            >
              <div className="flex flex-col w-full gap-2 h-[130px]">
                🔗 Testnet
                <p className="text-sm font-normal max-w-xs whitespace-normal block">
                  More complex and realistic. You need to fund your wallet
                  manually to play the game.
                </p>
              </div>
            </Button>
          </CardContent>
        </Card>
      </div>
    );
  }

  if (!gameRole) {
    return (
      <div className="space-y-6">
        <GameRoleSelector onRoleSelect={setGameRole} />
      </div>
    );
  }

  return (
    <div className="container mx-auto p-6 max-w-4xl">
      <Card>
        <CardHeader>
          <CardTitle className="text-2xl">
            {gameRole === GameRole.Player1
              ? "➕ Player 1 - Add Numbers"
              : "🤝 Player 2 - Add Numbers"}
          </CardTitle>
          <CardDescription>
            {gameRole === GameRole.Player1
              ? "Create the game and choose the numbers to add"
              : "Join the game and answer the sum"}
          </CardDescription>
        </CardHeader>

        <CardContent className="space-y-6">
          <NetworkInfo networkSelected={networkSelected} />
          <WalletSection networkSelected={networkSelected} />
          <PeerConnectionInfo />
          <PeerConnectionInput networkSelected={networkSelected} />
          {/* Game UUID Section */}
          {gameRole === GameRole.Player2 && <GameUUIDInput />}

          {gameRole === GameRole.Player1 ? (
            <>
              <Player1GameSetup />
              <GameStartNotification
                onStartGame={() => setIsGameStarted(true)}
                isPlayer1={true}
                isGameStarted={isGameStarted}
              />
            </>
          ) : (
            <Player2GameSetup />
          )}

          {gameState === GameState.WaitingResponse &&
            gameRole === GameRole.Player1 && (
              <GameActions
                onAccept={() => {}}
                onChallenge={() => {}}
                isLoading={false}
              />
            )}
        </CardContent>
      </Card>
    </div>
  );
}
