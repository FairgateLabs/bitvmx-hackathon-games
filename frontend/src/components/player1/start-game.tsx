import { Button } from "@/components/ui/button";
import { Play } from "lucide-react";

export function StartGame() {
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
        onClick={() => {
          /* nextGameState(GameState.WaitingAnswer) */
        }}
        className="w-full bg-blue-600 hover:bg-blue-700"
      >
        <Play className="h-4 w-4 mr-2" />
        Start Game
      </Button>
    </div>
  );
}
