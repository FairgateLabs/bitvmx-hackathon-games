import { Trophy } from "lucide-react";
import { BlocksRemaining } from "../ui/block-remaining";

export function WaitingAnswer() {
  const handleTimeout = () => {};

  return (
    <div className="p-4 bg-gray-50 border border-gray-200 rounded-lg">
      <div className="flex items-center gap-3 mb-3">
        <Trophy className="h-6 w-6 text-gray-600" />
        <h3 className="text-lg font-semibold text-gray-800">
          Waiting for Player 2 to answer
        </h3>
      </div>
      <p className="text-sm text-gray-700 mb-3">
        Player 2 is answering, please wait. In case of timeout, you will win the
        game.
      </p>
      <BlocksRemaining numberBlocks={24} onTimeout={handleTimeout} />
    </div>
  );
}
