"use client";

import { useState } from "react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Separator } from "@/components/ui/separator";
import { useAddNumbersGame } from "@/hooks/useAddNumbersGame";

// Types
interface WalletInfo {
  address: string;
  balance: number;
  network: "regtest" | "testnet";
}

interface GameNumbers {
  number1?: number;
  number2?: number;
}

export default function AddNumbersPage() {
  const [gameMode, setGameMode] = useState<"player1" | "player2" | null>(null);
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

  if (!gameMode) {
    return (
      <div className="container mx-auto p-6 max-w-2xl">
        <Card>
          <CardHeader>
            <CardTitle className="text-2xl text-center">
              🎮 Add Numbers Game
            </CardTitle>
            <CardDescription className="text-center">
              Choose the role you want to play
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="text-center mb-6">
              <p className="text-muted-foreground">
                Two players compete by adding numbers. Who are you?
              </p>
            </div>

            <div className="grid grid-cols-2 gap-4">
              <Button
                onClick={() => setGameMode("player1")}
                className="h-24 text-lg"
                variant="outline"
              >
                🎯 Player 1<br />
                <span className="text-sm font-normal">Create the game</span>
              </Button>

              <Button
                onClick={() => setGameMode("player2")}
                className="h-24 text-lg"
                variant="outline"
              >
                🎮 Player 2<br />
                <span className="text-sm font-normal">Join the game</span>
              </Button>
            </div>
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="container mx-auto p-6 max-w-4xl">
      <Card>
        <CardHeader>
          <CardTitle className="text-2xl">
            {gameMode === "player1"
              ? "🎯 Player 1 - Add Numbers"
              : "🎮 Player 2 - Add Numbers"}
          </CardTitle>
          <CardDescription>
            {gameMode === "player1"
              ? "Create the game and choose the numbers to add"
              : "Join the game and guess the sum"}
          </CardDescription>
        </CardHeader>

        <CardContent className="space-y-6">
          {/* Wallet Information */}
          <WalletSection walletInfo={walletInfo} />

          <Separator />

          {/* Game Setup */}
          {gameMode === "player1" ? (
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
          {gameState === "waiting_response" && gameMode === "player1" && (
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

function WalletSection({ walletInfo }: { walletInfo: WalletInfo | null }) {
  if (!walletInfo) return null;

  return (
    <div className="p-4 bg-muted/50 rounded-lg">
      <h3 className="font-semibold mb-2">💰 Wallet Information</h3>
      <div className="grid grid-cols-2 gap-4 text-sm">
        <div>
          <Label>Address:</Label>
          <p className="font-mono text-xs break-all">{walletInfo.address}</p>
        </div>
        <div>
          <Label>Balance:</Label>
          <p className="font-semibold">{walletInfo.balance} BTC</p>
        </div>
      </div>
    </div>
  );
}

interface Player1GameSetupProps {
  numbers: GameNumbers;
  setNumbers: (numbers: GameNumbers) => void;
  generateProgram: () => void;
  gameId: string;
  gameState: string;
  isLoading: boolean;
}

function Player1GameSetup({
  numbers,
  setNumbers,
  generateProgram,
  gameId,
  gameState,
  isLoading,
}: Player1GameSetupProps) {
  return (
    <div className="space-y-4">
      <h3 className="text-lg font-semibold">🎲 Game Setup</h3>

      {!gameId ? (
        <div className="space-y-4">
          <div className="grid grid-cols-2 gap-4">
            <div>
              <Label htmlFor="number1">First Number</Label>
              <Input
                id="number1"
                type="number"
                value={numbers.number1 || ""}
                onChange={(e) =>
                  setNumbers({ ...numbers, number1: parseInt(e.target.value) })
                }
                placeholder="Eg: 5"
              />
            </div>
            <div>
              <Label htmlFor="number2">Second Number</Label>
              <Input
                id="number2"
                type="number"
                value={numbers.number2 || ""}
                onChange={(e) =>
                  setNumbers({ ...numbers, number2: parseInt(e.target.value) })
                }
                placeholder="Eg: 3"
              />
            </div>
          </div>

          <Button
            onClick={generateProgram}
            disabled={!numbers.number1 || !numbers.number2 || isLoading}
            className="w-full"
          >
            {isLoading ? "Generating..." : "🚀 Generate Program"}
          </Button>
        </div>
      ) : (
        <div className="p-4 bg-green-50 border border-green-200 rounded-md">
          <h4 className="font-semibold text-green-800 mb-2">
            ✅ Program Generated
          </h4>
          <p className="text-sm text-green-700 mb-2">Game ID:</p>
          <p className="font-mono text-xs bg-green-100 p-2 rounded break-all">
            {gameId}
          </p>
          <p className="text-sm text-green-700 mt-2">
            Share this ID with Player 2 to join the game.
          </p>
        </div>
      )}

      {gameState === "waiting_response" && (
        <div className="p-4 bg-blue-50 border border-blue-200 rounded-md">
          <h4 className="font-semibold text-blue-800">
            ⏳ Waiting for Player 2&apos;s response...
          </h4>
          <p className="text-sm text-blue-700">
            Player 2 is calculating the sum of {numbers.number1} +{" "}
            {numbers.number2}
          </p>
        </div>
      )}
    </div>
  );
}

interface Player2GameSetupProps {
  gameId: string;
  setGameId: (id: string) => void;
  peerIP: string;
  setPeerIP: (ip: string) => void;
  peerPort: string;
  setPeerPort: (port: string) => void;
  submitAnswer: (answer: string) => void;
  isLoading: boolean;
}

function Player2GameSetup({
  gameId,
  setGameId,
  peerIP,
  setPeerIP,
  peerPort,
  setPeerPort,
  submitAnswer,
  isLoading,
}: Player2GameSetupProps) {
  const [answer, setAnswer] = useState("");

  return (
    <div className="space-y-4">
      <h3 className="text-lg font-semibold">🔗 Join the Game</h3>

      {!gameId ? (
        <div className="space-y-4">
          <div>
            <Label htmlFor="gameId">Game ID</Label>
            <Input
              id="gameId"
              value={gameId}
              onChange={(e) => setGameId(e.target.value)}
              placeholder="Enter the game UUID"
            />
          </div>

          <div className="grid grid-cols-2 gap-4">
            <div>
              <Label htmlFor="peerIP">Player 1&apos;s IP</Label>
              <Input
                id="peerIP"
                value={peerIP}
                onChange={(e) => setPeerIP(e.target.value)}
                placeholder="Eg: 192.168.1.100"
              />
            </div>
            <div>
              <Label htmlFor="peerPort">Port</Label>
              <Input
                id="peerPort"
                value={peerPort}
                onChange={(e) => setPeerPort(e.target.value)}
                placeholder="Eg: 8080"
              />
            </div>
          </div>

          <Button
            onClick={() => {
              /* TODO: Implement join game */
            }}
            disabled={!gameId || !peerIP || !peerPort}
            className="w-full"
          >
            🎮 Join the Game
          </Button>
        </div>
      ) : (
        <div className="space-y-4">
          <div className="p-4 bg-blue-50 border border-blue-200 rounded-md">
            <h4 className="font-semibold text-blue-800">
              🎯 Calculate the Sum
            </h4>
            <p className="text-sm text-blue-700">
              Player 1 has chosen two numbers. What is the sum?
            </p>
          </div>

          <div>
            <Label htmlFor="answer">Your Answer</Label>
            <Input
              id="answer"
              type="number"
              value={answer}
              onChange={(e) => setAnswer(e.target.value)}
              placeholder="Enter the sum"
            />
          </div>

          <Button
            onClick={() => submitAnswer(answer)}
            disabled={!answer || isLoading}
            className="w-full"
          >
            {isLoading ? "Sending..." : "📤 Send Answer"}
          </Button>
        </div>
      )}
    </div>
  );
}

interface GameActionsProps {
  onAccept: () => void;
  onChallenge: () => void;
  isLoading: boolean;
}

function GameActions({ onAccept, onChallenge, isLoading }: GameActionsProps) {
  return (
    <div className="space-y-4">
      <h3 className="text-lg font-semibold">🎯 Game Actions</h3>
      <p className="text-sm text-muted-foreground">
        Player 2 has sent their answer. What do you want to do?
      </p>

      <div className="grid grid-cols-2 gap-4">
        <Button
          onClick={onAccept}
          disabled={isLoading}
          className="bg-green-600 hover:bg-green-700"
        >
          ✅ Correct - Transfer Funds
        </Button>

        <Button
          onClick={onChallenge}
          disabled={isLoading}
          variant="destructive"
        >
          ⚖️ Challenge - Initiate Dispute
        </Button>
      </div>

      <p className="text-xs text-muted-foreground text-center">
        ⏰ If you do nothing, Player 2 will automatically win by timeout
      </p>
    </div>
  );
}
