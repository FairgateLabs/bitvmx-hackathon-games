import { Trophy } from "lucide-react";
import { TimeRemaining } from "../ui/time-remaining";
// import { useNextGameState } from "@/hooks/useGameState";
import { GameState } from "@/types/game";

export function WaitingAnswer() {
  // const { mutate: nextGameState } = useNextGameState();
  const handleTimeout = () => {
    // nextGameState(GameState.ChooseAction);
  };

  return (
    <div className="p-4 bg-gray-50 border border-gray-200 rounded-lg">
      <div className="flex items-center gap-3 mb-3">
        <Trophy className="h-6 w-6 text-gray-600" />
        <h3 className="text-lg font-semibold text-gray-800">
          Waiting for Player 2 to answer
        </h3>
      </div>
      <p className="text-sm text-gray-700 mb-3">
        Player 2 is answering, please wait.
      </p>
      <TimeRemaining numberBlocks={5} size="lg" onTimeout={handleTimeout} />
    </div>
  );
}
