"use client";

import { useState } from "react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import { useAddNumbersGame } from "@/hooks/useAddNumbersGame";
import { GameRoleSelector, GameRole } from "@/components/game-role-selector";
import { WalletSection } from "@/components/wallet-section";
import { Player1GameSetup } from "@/components/player1-game-setup";
import { Player2GameSetup } from "@/components/player2-game-setup";
import { GameActions } from "@/components/game-actions";
import { NetworkInfo } from "@/components/network-info";
import { PlayerConnectionInfo } from "@/components/player-connection-info";
import { PeerConnectionInput } from "@/components/peer-connection-input";
import { GameUUIDGenerator } from "@/components/game-uuid-generator";
import { GameUUIDInput } from "@/components/game-uuid-input";
import { Button } from "@/components/ui/button";
import { NetworkType } from "@/types/network";

// Types
interface WalletInfo {
  address: string;
  balance: number;
  network: NetworkType;
}

interface GameNumbersToAdd {
  number1?: number;
  number2?: number;
}

export default function AddNumbersPage() {
  const [gameRole, setGameRole] = useState<GameRole | null>(null);
  const [gameId, setGameId] = useState("");
  const [peerIP, setPeerIP] = useState("");
  const [peerPort, setPeerPort] = useState("");

  const {
    walletInfo,
    gameState,
    numbers,
    setNumbers,
    generateProgram,
    submitAnswer,
    acceptAnswer,
    challengeAnswer,
    isLoading,
    error,
  } = useAddNumbersGame();

  const [networkSelected, setNetworkSelected] = useState<NetworkType | null>(
    null
  );
  const [peerConnection, setPeerConnection] = useState<{
    ip: string;
    port: string;
  } | null>(null);
  const [gameUUID, setGameUUID] = useState<string>("");

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
              : "Join the game and guess the sum"}
          </CardDescription>
        </CardHeader>

        <CardContent className="space-y-6">
          <NetworkInfo networkSelected={networkSelected} />

          <PlayerConnectionInfo networkSelected={networkSelected} />

          <PeerConnectionInput
            networkSelected={networkSelected}
            onConnectionSet={(ip, port) => setPeerConnection({ ip, port })}
          />

          {walletInfo && (
            <WalletSection
              walletInfo={walletInfo}
              networkSelected={networkSelected}
            />
          )}

          {/* Game UUID Section */}
          {gameRole === GameRole.Player1 ? (
            <GameUUIDGenerator onUUIDGenerated={setGameUUID} />
          ) : (
            <GameUUIDInput onUUIDEntered={setGameUUID} />
          )}

          {/* Connection Validation */}
          {!peerConnection && (
            <div className="p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
              <h3 className="font-semibold mb-2 text-yellow-800">
                ⚠️ Connection Setup Required
              </h3>
              <p className="text-sm text-yellow-700">
                Please complete the connection setup by entering the other
                player's IP address and port above.
              </p>
            </div>
          )}

          {!gameUUID && (
            <div className="p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
              <h3 className="font-semibold mb-2 text-yellow-800">
                ⚠️ Game UUID Required
              </h3>
              <p className="text-sm text-yellow-700">
                {gameRole === GameRole.Player1
                  ? "Please generate a game UUID above to share with Player 2."
                  : "Please enter the game UUID provided by Player 1 above."}
              </p>
            </div>
          )}

          <Separator />

          {/* Game Setup */}
          {peerConnection && gameUUID ? (
            gameRole === GameRole.Player1 ? (
              <Player1GameSetup
                numbers={numbers}
                setNumbers={setNumbers}
                generateProgram={generateProgram}
                gameId={gameId}
                gameState={gameState}
                isLoading={isLoading}
              />
            ) : (
              <Player2GameSetup
                gameId={gameId}
                setGameId={setGameId}
                peerIP={peerIP}
                setPeerIP={setPeerIP}
                peerPort={peerPort}
                setPeerPort={setPeerPort}
                submitAnswer={submitAnswer}
                isLoading={isLoading}
              />
            )
          ) : (
            <div className="p-4 bg-blue-50 border border-blue-200 rounded-lg">
              <h3 className="font-semibold mb-2 text-blue-800">
                ⏳ Setup Complete
              </h3>
              <p className="text-sm text-blue-700">
                Once you've completed the connection setup and UUID exchange,
                the game will be ready to start.
              </p>
            </div>
          )}

          {/* Game Actions */}
          {gameState === "waiting_response" &&
            gameRole === GameRole.Player1 && (
              <GameActions
                onAccept={acceptAnswer}
                onChallenge={challengeAnswer}
                isLoading={isLoading}
              />
            )}

          {/* Error Display */}
          {error && (
            <div className="p-4 bg-destructive/10 border border-destructive rounded-md">
              <p className="text-destructive">{error}</p>
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
