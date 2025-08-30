import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import { Play, Clock, Trophy, Check, X } from "lucide-react";
import { usePlayer2Answer } from "@/hooks/usePlayer2Response";
import { YouLost } from "./you-lost";
import { YouWin } from "./you-win";
import { useNextGameState } from "@/hooks/useGameState";
import { useGameState } from "@/hooks/useGameState";
import { GameState } from "@/types/gameState";

export function StartGame() {
  const [timeLeft, setTimeLeft] = useState(60); // 60 seconds timeout
  const [isTimeoutRunning, setIsTimeoutRunning] = useState(false);
  const [player1Won, setPlayer1Won] = useState<boolean | null>(null);
  const [hasResponded, setHasResponded] = useState(false);
  const { mutate: nextGameState } = useNextGameState();
  const { data: gameState } = useGameState();

  let gameId = "123";
  // Fetch Player 2's response
  const { data: player2Response, isLoading: isLoadingResponse } =
    usePlayer2Answer(gameId);

  useEffect(() => {
    let interval: NodeJS.Timeout;

    if (isTimeoutRunning && timeLeft > 0) {
      interval = setInterval(() => {
        setTimeLeft((prev) => {
          if (prev <= 1) {
            // Timeout reached
            if (gameState === GameState.ChooseAction) {
              // Player 1 accepted the answer, so they lose
              setPlayer1Won(false);
            } else {
              // Player 1 wins by default (no response or challenged)
              setPlayer1Won(true);
            }
            setIsTimeoutRunning(false);
            return 0;
          }
          return prev - 1;
        });
      }, 1000);
    }

    return () => {
      if (interval) {
        clearInterval(interval);
      }
    };
  }, [isTimeoutRunning, timeLeft]);

  const handleStartGame = () => {
    setIsTimeoutRunning(true);
    nextGameState(null);
  };

  if (player1Won !== null) {
    if (player1Won) {
      return <YouWin />;
    } else {
      return <YouLost />;
    }
  }

  return (
    <div className="p-4 bg-blue-50 border border-blue-200 rounded-lg">
      <div className="flex items-center gap-3 mb-3">
        <Play className="h-6 w-6 text-blue-600" />
        <h3 className="text-lg font-semibold text-blue-800">
          🚀 Ready to Start
        </h3>
      </div>
      <p className="text-sm text-blue-700 mb-4">
        Everything is complete! The program is finally set up and ready to go.
      </p>

      <div className="p-3 bg-white rounded-lg mb-4">
        <h4 className="font-semibold text-blue-800 mb-2">
          ⚠️ Game Start Information
        </h4>
        <ul className="text-sm text-blue-700 space-y-1">
          <li>
            The first transaction is submitted on-chain by you when you click
            the Start Game button
          </li>
          <li>
            The game starts and the timer begins for Player 2. Player 2 has 1
            minute to respond.
          </li>
          <li>
            If Player 2 does not respond in time, you automatically win due to
            the transaction timeout
          </li>
        </ul>
      </div>
      <Button
        onClick={handleStartGame}
        className="w-full bg-blue-600 hover:bg-blue-700"
      >
        <Play className="h-4 w-4 mr-2" />
        Start Game
      </Button>
    </div>
  );
}
