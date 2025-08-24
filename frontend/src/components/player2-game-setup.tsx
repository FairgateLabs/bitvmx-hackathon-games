import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

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

export function Player2GameSetup({
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
            className="w-full cursor-pointer"
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
            className="w-full cursor-pointer"
          >
            {isLoading ? "Sending..." : "📤 Send Answer"}
          </Button>
        </div>
      )}
    </div>
  );
}
