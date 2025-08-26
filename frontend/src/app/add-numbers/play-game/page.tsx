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
import { Button } from "@/components/ui/button";

// Types
interface WalletInfo {
  address: string;
  balance: number;
  network: "regtest" | "testnet";
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

  const [networkSelected, setNetworkSelected] = useState(false);

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
              onClick={() => setNetworkSelected(true)}
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
              onClick={() => setNetworkSelected(true)}
              className="flex text-center h-40 text-lg cursor-pointer"
              variant="outline"
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
          {/* Wallet Information */}
          <WalletSection walletInfo={walletInfo} />

          <Separator />

          {/* Game Setup */}
          {gameRole === GameRole.Player1 ? (
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
