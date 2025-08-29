import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import { Play, Clock, Trophy, Check, X } from "lucide-react";
import { usePlayer2Answer } from "@/hooks/usePlayer2Response";

export function StartGame() {
  const [timeLeft, setTimeLeft] = useState(60); // 60 seconds timeout
  const [isTimeoutRunning, setIsTimeoutRunning] = useState(false);
  const [player1Won, setPlayer1Won] = useState<boolean | null>(null);
  const [hasResponded, setHasResponded] = useState(false);
  const [chosenAction, setChosenAction] = useState<
    "accept" | "challenge" | null
  >(null);

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
            if (chosenAction === "accept") {
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
  }, [isTimeoutRunning, timeLeft, chosenAction]);

  const handleStartGame = () => {
    setIsTimeoutRunning(true);
  };

  const handleAcceptAnswer = () => {
    setChosenAction("accept");
    setHasResponded(true);
    // Player 1 accepts but will lose when timeout expires
  };

  const handleChallengeAnswer = () => {
    setChosenAction("challenge");
    setHasResponded(true);
    setPlayer1Won(true);
    setIsTimeoutRunning(false);
    // Player 1 challenges and wins
  };

  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}:${secs.toString().padStart(2, "0")}`;
  };

  if (player1Won !== null) {
    if (player1Won) {
      return (
        <div className="p-4 bg-green-50 border border-green-200 rounded-lg">
          <div className="flex items-center gap-3 mb-3">
            <Trophy className="h-6 w-6 text-green-600" />
            <h3 className="text-lg font-semibold text-green-800">
              🎉 You Win!
            </h3>
          </div>
          <p className="text-sm text-green-700 mb-3">
            {chosenAction === "challenge"
              ? "You challenged Player 2's answer and won!"
              : "Player 2 did not answer within the time limit!"}
          </p>
          <p className="text-xs text-green-600">
            The game has ended.{" "}
            {chosenAction === "challenge"
              ? "Your challenge was successful."
              : "Player 2's timeout expired."}
          </p>
        </div>
      );
    } else {
      return (
        <div className="p-4 bg-red-50 border border-red-200 rounded-lg">
          <div className="flex items-center gap-3 mb-3">
            <Trophy className="h-6 w-6 text-red-600" />
            <h3 className="text-lg font-semibold text-red-800">😔 You Lost!</h3>
          </div>
          <p className="text-xs text-red-600">
            The game has ended. Player 2 wins because you accepted their answer.
          </p>
        </div>
      );
    }
  }

  if (isTimeoutRunning) {
    // Check if Player 2 has responded
    if (player2Response && !hasResponded) {
      return (
        <div className="p-4 bg-blue-50 border border-blue-200 rounded-lg">
          <div className="flex items-center gap-3 mb-3">
            <Check className="h-6 w-6 text-blue-600" />
            <h3 className="text-lg font-semibold text-blue-800">
              🎯 Player 2 Responded!
            </h3>
          </div>
          <p className="text-sm text-blue-700 mb-4">
            Player 2 answered with the sum:{" "}
            <strong>{player2Response.sum}</strong>
          </p>

          <div className="p-3 bg-white rounded-lg mb-4">
            <h4 className="font-semibold text-blue-800 mb-2">
              ⚠️ Choose Your Action
            </h4>
            <p className="text-sm text-blue-700 mb-3">
              Based on Player 2's answer, you have three options:
            </p>
            <ul className="text-sm text-blue-700 space-y-2 mb-4">
              <li className="flex items-start gap-2">
                <span className="text-green-600">✅</span>
                <span>
                  <strong>"Accept Player 2's answer"</strong> → Wait for the
                  timeout and lose.
                </span>
              </li>
              <li className="flex items-start gap-2">
                <span className="text-red-600">❌</span>
                <span>
                  <strong>"I disagree – I win"</strong> → Challenge Player 2's
                  answer.
                </span>
              </li>
              <li className="flex items-start gap-2">
                <span className="text-gray-600">⏰</span>
                <span>
                  <strong>Do nothing</strong> → Let the timeout expire and lose.
                </span>
              </li>
            </ul>
          </div>

          <div className="grid grid-cols-2 gap-3">
            <Button
              onClick={handleAcceptAnswer}
              className="bg-green-600 hover:bg-green-700"
            >
              <Check className="h-4 w-4 mr-2" />
              Accept Answer
            </Button>
            <Button
              onClick={handleChallengeAnswer}
              className="bg-red-600 hover:bg-red-700"
            >
              <X className="h-4 w-4 mr-2" />
              Challenge Answer
            </Button>
          </div>

          <p className="text-xs text-blue-600 mt-3 text-center">
            Time remaining:{" "}
            <span className="font-mono font-bold">{formatTime(timeLeft)}</span>
          </p>
        </div>
      );
    }

    // Player 2 hasn't responded yet or Player 1 has already made a choice
    return (
      <div className="p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
        <div className="flex items-center gap-3 mb-3">
          <Clock className="h-6 w-6 text-yellow-600" />
          <h3 className="text-lg font-semibold text-yellow-800">
            ⏰ Timeout Running
          </h3>
        </div>
        <p className="text-sm text-yellow-700 mb-3">
          {hasResponded && chosenAction === "accept"
            ? "You accepted Player 2's answer. Waiting for timeout to expire..."
            : isLoadingResponse
            ? "Waiting for Player 2 to respond..."
            : "Waiting for Player 2's answer. Time remaining:"}
        </p>
        <div className="text-center mb-3">
          <div className="text-2xl font-mono font-bold text-yellow-800">
            {formatTime(timeLeft)}
          </div>
        </div>
        <p className="text-xs text-yellow-600">
          {hasResponded && chosenAction === "accept"
            ? "You will lose when the timeout expires because you accepted the answer."
            : isLoadingResponse
            ? "Player 2 is calculating their answer..."
            : "If Player 2 doesn't answer in time, you will win automatically because of transaction timeout."}
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

      {!isTimeoutRunning ? (
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
              <li>
                The game starts and the timer begins for Player 2. Player 2 has
                1 minute to respond.
              </li>
              <li>
                If Player 2 does not respond in time, you automatically win due
                to the transaction timeout
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
