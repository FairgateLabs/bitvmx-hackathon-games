import { Trophy } from "lucide-react";
import { TimeRemaining } from "../ui/time-remaining";
import { useNextGameState } from "@/hooks/useGameState";
import { GameState } from "@/types/gameState";

export function WaitingAnswer() {
  const { mutate: nextGameState } = useNextGameState();
  const handleTimeout = () => {
    nextGameState(GameState.ChooseAction);
  };

  return (
    <div className="p-4 bg-green-50 border rounded-lg">
      <div className="flex items-center gap-3 mb-3">
        <Trophy className="h-6 w-6 " />
        <h3 className="text-lg font-semibold">
          Waiting for Player 2 to answer
        </h3>
      </div>
      <p className="text-sm  mb-3">
        Player 2 has not yet answered the protocol. Please wait for them to
        answer.
      </p>
      <TimeRemaining numberBlocks={2} size="lg" onTimeout={handleTimeout} />
    </div>
  );
}
