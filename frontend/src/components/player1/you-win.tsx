import { Trophy } from "lucide-react";
import { useGameState } from "@/hooks/useGameState";
import { GameState } from "@/types/gameState";

export function YouWin() {
  const { data: gameState } = useGameState();
  return (
    <div className="p-4 bg-green-50 border border-green-200 rounded-lg">
      <div className="flex items-center gap-3 mb-3">
        <Trophy className="h-6 w-6 text-green-600" />
        <h3 className="text-lg font-semibold text-green-800">🎉 You Win!</h3>
      </div>
      <p className="text-sm text-green-700 mb-3">
        {gameState === GameState.ChallengeAnswer
          ? "You challenged Player 2's answer and won!"
          : "Player 2 did not answer within the time limit!"}
      </p>
      <p className="text-xs text-green-600">
        The game has ended.{" "}
        {gameState === GameState.ChallengeAnswer
          ? "Your challenge was successful."
          : "Player 2's timeout expired."}
      </p>
    </div>
  );
}
