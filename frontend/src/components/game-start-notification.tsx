import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import { Play, Clock, Trophy } from "lucide-react";

interface GameStartNotificationProps {
  onStartGame: () => void;
  isPlayer1: boolean;
  isGameStarted?: boolean;
}

export function GameStartNotification({
  onStartGame,
  isPlayer1,
  isGameStarted = false,
}: GameStartNotificationProps) {
  const [timeLeft, setTimeLeft] = useState(60); // 60 seconds timeout
  const [isTimeoutRunning, setIsTimeoutRunning] = useState(false);
  const [player1Won, setPlayer1Won] = useState(false);

  useEffect(() => {
    let interval: NodeJS.Timeout;

    if (isGameStarted && isPlayer1 && isTimeoutRunning && timeLeft > 0) {
      interval = setInterval(() => {
        setTimeLeft((prev) => {
          if (prev <= 1) {
            // Timeout reached - Player 1 wins
            setPlayer1Won(true);
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
  }, [isGameStarted, isPlayer1, isTimeoutRunning, timeLeft]);

  const handleStartGame = () => {
    onStartGame();
    setIsTimeoutRunning(true);
  };

  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}:${secs.toString().padStart(2, "0")}`;
  };

  if (player1Won) {
    return (
      <div className="p-4 bg-green-50 border border-green-200 rounded-lg">
        <div className="flex items-center gap-3 mb-3">
          <Trophy className="h-6 w-6 text-green-600" />
          <h3 className="text-lg font-semibold text-green-800">🎉 You Win!</h3>
        </div>
        <p className="text-sm text-green-700 mb-3">
          Player 2 did not answer within the time limit!
        </p>
        <p className="text-xs text-green-600">
          The game has ended. Player 2's timeout expired.
        </p>
      </div>
    );
  }

  if (isGameStarted && isPlayer1 && isTimeoutRunning) {
    return (
      <div className="p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
        <div className="flex items-center gap-3 mb-3">
          <Clock className="h-6 w-6 text-yellow-600" />
          <h3 className="text-lg font-semibold text-yellow-800">
            ⏰ Timeout Running
          </h3>
        </div>
        <p className="text-sm text-yellow-700 mb-3">
          Waiting for Player 2's answer. Time remaining:
        </p>
        <div className="text-center mb-3">
          <div className="text-2xl font-mono font-bold text-yellow-800">
            {formatTime(timeLeft)}
          </div>
        </div>
        <p className="text-xs text-yellow-600">
          If Player 2 doesn't answer in time, you will win automatically because
          transaction timeout.
        </p>
      </div>
    );
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

      {!isGameStarted ? (
        <>
          <div className="p-3 bg-white rounded-lg mb-4">
            <h4 className="font-semibold text-blue-800 mb-2">
              ⚠️ Game Start Information
            </h4>
            <ul className="text-sm text-blue-700 space-y-1">
              <li>
                The first transaction is submitted on-chain by you when you
                click the Start Game button
              </li>
              <li> The game starts and the timer begins for Player 2</li>
              <li> Player 2 has 1 minute to respond</li>
              <li>
                If they fail to respond in time, you automatically win due to
                the transaction timeout
              </li>
            </ul>
          </div>
          <Button
            onClick={handleStartGame}
            className="w-full bg-blue-600 hover:bg-blue-700 cursor-pointer"
          >
            <Play className="h-4 w-4 mr-2" />
            Start Game
          </Button>
        </>
      ) : (
        <div className="p-3 bg-green-100 border border-green-300 rounded-lg">
          <div className="flex items-center gap-2 mb-2">
            <Play className="h-4 w-4 text-green-600" />
            <span className="font-semibold text-green-800">Game Started!</span>
          </div>
          <p className="text-sm text-green-700">
            The first transaction has been triggered on-chain. The game is now
            active and waiting for Player 2's response...
          </p>
        </div>
      )}
    </div>
  );
}
