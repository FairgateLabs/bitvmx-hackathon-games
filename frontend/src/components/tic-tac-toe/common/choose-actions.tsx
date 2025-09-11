import { Button } from "@/components/ui/button";
import { TimeRemaining } from "@/components/ui/time-remaining";
import { useCurrentGame } from "@/hooks/useGame";
import { GameEndResult } from "./tic-tac-toe-board";
import { useState } from "react";

export function ChooseAction({ winner, isTimeout }: GameEndResult) {
  const { data: currentGame } = useCurrentGame();
  const currentPlayer = currentGame?.role;
  const [buttonsDisabled, setButtonsDisabled] = useState(false);

  // Only show actions when game is finished and player can challenge/accept
  // Not during timeout scenarios - only when other player wins or draw
  const canChallengeOrAccept =
    !isTimeout && (winner !== currentPlayer || winner === null);

  const handleAccept = () => {
    if (isTimeout) {
      // Timeout scenario - winner is determined by timeout
      if (winner === currentPlayer) {
      } else {
      }
    } else if (winner === currentPlayer) {
      // Player won normally - accept victory
    } else if (winner === null) {
      // Draw - accept draw (using accept state for now)
    } else {
      // Player lost normally - accept loss
    }
  };

  const handleChallenge = () => {
    // Any result can be challenged
  };

  const handleTimeout = () => {
    // Disable all buttons when timeout occurs
    setButtonsDisabled(true);

    // This handles the timeout for the action selection phase
    if (winner === currentPlayer) {
    } else if (winner === null) {
    } else {
    }
  };

  return (
    <div className="space-y-4 border border-gray-200 rounded-lg p-4">
      <h3 className="text-lg font-semibold">🎯 Tic Tac Toe Game Result</h3>

      <div>
        <p className="text-sm font-medium text-gray-800 mb-2">
          {isTimeout &&
            winner === currentPlayer &&
            `🎉 Congratulations! You won because the opponent timed out!`}
          {isTimeout &&
            winner !== currentPlayer &&
            `😔 You lost because you timed out.`}
          {!isTimeout &&
            winner === currentPlayer &&
            `🎉 Congratulations! You won the tic-tac-toe game!`}
          {!isTimeout &&
            winner !== currentPlayer &&
            winner !== null &&
            `😔 You lost the tic-tac-toe game.`}
          {!isTimeout &&
            winner === null &&
            `🤝 The tic-tac-toe game ended in a draw.`}
        </p>
      </div>

      {canChallengeOrAccept && (
        <div className="text-center mt-3">
          <p className="text-xs text-muted-foreground text-center mb-2">
            {winner === currentPlayer &&
              "⏰ If the opponent does nothing, you will automatically win by timeout"}
            {winner !== currentPlayer &&
              winner !== null &&
              "⏰ If you do nothing, you will automatically lose by timeout"}
            {winner === null &&
              "⏰ If no action is taken, the game will be considered a draw"}
          </p>
          <TimeRemaining numberBlocks={2} onTimeout={handleTimeout} size="lg" />
        </div>
      )}

      {canChallengeOrAccept && (
        <div className="p-3">
          <h4 className="font-semibold text-blue-800 mb-2">
            ⚠️ Choose Your Action
          </h4>
          <p className="text-sm text-blue-700 mb-3">
            Based on the game result, you have the following options:
          </p>
          <ul className="text-sm text-blue-700 space-y-2 mb-4">
            {winner !== currentPlayer && winner !== null && (
              <>
                <li className="flex items-start gap-2">
                  <span className="text-green-600">✅</span>
                  <span>
                    <strong>&quot;Accept Loss&quot;</strong> → Accept the game
                    result and lose
                  </span>
                </li>
                <li className="flex items-start gap-2">
                  <span className="text-red-600">⚖️</span>
                  <span>
                    <strong>&quot;Dispute Result&quot;</strong> → Challenge the
                    game result
                  </span>
                </li>
              </>
            )}
            {winner === null && (
              <>
                <li className="flex items-start gap-2">
                  <span className="text-green-600">✅</span>
                  <span>
                    <strong>&quot;Accept Draw&quot;</strong> → Accept the draw
                    result
                  </span>
                </li>
                <li className="flex items-start gap-2">
                  <span className="text-red-600">⚖️</span>
                  <span>
                    <strong>&quot;Dispute Result&quot;</strong> → Challenge the
                    draw result
                  </span>
                </li>
              </>
            )}
            <li className="flex items-start gap-2">
              <span className="text-gray-600">⏰</span>
              <span>
                <strong>Do nothing</strong> → Let the timeout expire
              </span>
            </li>
          </ul>
        </div>
      )}

      {canChallengeOrAccept && (
        <div className="grid grid-cols-2 gap-4">
          <Button
            onClick={handleAccept}
            disabled={buttonsDisabled}
            className="bg-green-600 hover:bg-green-700 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            ✅ {winner === null ? "Accept Draw" : "Accept Loss"}
          </Button>

          <Button
            onClick={handleChallenge}
            disabled={buttonsDisabled}
            variant="destructive"
            className="cursor-pointer bg-orange-600 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            ⚖️ Dispute Result
          </Button>
        </div>
      )}
    </div>
  );
}
