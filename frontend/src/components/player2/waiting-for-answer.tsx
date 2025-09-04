import { TimeRemaining } from "@/components/ui/time-remaining";
import { useNextGameState } from "@/hooks/useGameState";
import { GameState } from "@/types/game";

export function WaitingForAnswer() {
  const { mutate: nextGameState } = useNextGameState();
  const handleTimeout = () => {
    console.log("handleTimeout");
    nextGameState(GameState.GameCompleteYouWinByChallenge);
  };
  return (
    <div className="space-y-4 border border-gray-200 rounded-lg p-4 bg-blue-50">
      <h3 className="text-lg font-semibold text-blue-800 text-center">
        ⏳ Waiting for Player 1
      </h3>

      <div className="text-center space-y-4">
        <p className="text-lg text-blue-700 font-medium">
          Player 1 is making their decision...
        </p>

        <div className="p-4 bg-white rounded-lg border border-blue-200 max-w-md mx-auto">
          <p className="text-sm text-blue-700 text-center">
            Please wait while Player 1 decides whether to accept your answer,
            challenge it, or let the timeout expire.
          </p>
        </div>

        <p className="text-sm text-blue-600 bg-blue-100 px-4 py-2 rounded-lg">
          ⏰ The game will continue automatically once Player 1 responds!
        </p>

        <div className="text-center">
          <p className="text-sm text-red-600 mb-2">Time remaining:</p>
          <TimeRemaining numberBlocks={5} size="lg" onTimeout={handleTimeout} />
        </div>
      </div>
    </div>
  );
}
