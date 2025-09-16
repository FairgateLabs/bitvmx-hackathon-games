import { Play } from "lucide-react";
import { useState } from "react";
import { GameEndResult, TicTacToeBoard } from "./tic-tac-toe-board";
import { ChooseAction } from "./choose-actions";

export function StartGame() {
  const [gameResult, setGameResult] = useState<GameEndResult | null>(null);
  return (
    <div className="p-4 bg-gray-50 border border-gray-200 rounded-lg">
      <div className="flex items-center gap-3 mb-3">
        <h3 className="text-lg font-semibold text-gray-800">
          🚀 Ready to Start
        </h3>
      </div>
      <p className="text-sm text-gray-700 mb-4">
        Everything is complete! The Puzzle program is finally set up and ready
        to go.
      </p>

      <div className="p-3 bg-white rounded-lg mb-4">
        <h4 className="font-semibold text-gray-800 mb-2">
          ⚠️ Game Start Information
        </h4>
        <ul className="text-sm text-gray-700 space-y-1">
          <li>
            Each move submits a transaction on-chain, ensuring the game&apos;s
            integrity.
          </li>
          <li>
            The timer runs for both players on each move. If a player fails to
            move within the stipulated time, they lose the game.
          </li>
          <li>
            Stay alert and make your move promptly to avoid losing by timeout.
          </li>
        </ul>
      </div>

      <TicTacToeBoard
        onGameEnd={(gameResult) => {
          setGameResult(gameResult);
          console.log(gameResult);
        }}
      />
      {gameResult && (
        <ChooseAction
          winner={gameResult.winner}
          isTimeout={gameResult.isTimeout}
        />
      )}
    </div>
  );
}
