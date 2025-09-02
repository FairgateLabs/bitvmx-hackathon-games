import { Button } from "@/components/ui/button";
import { useGameState, useNextGameState } from "@/hooks/useGameState";
import { GameState } from "@/types/gameState";
import { TimeRemaining } from "@/components/ui/time-remaining";

export function ChooseAction() {
  const { mutate: nextGameState } = useNextGameState();

  const handleAccept = () => {
    // Logic for accepting the answer and transferring funds
    nextGameState(GameState.GameCompleteYouLoseByAccept);
  };

  const handleChallenge = () => {
    // Logic for challenging the answer and initiating a dispute
    nextGameState(GameState.ChallengeAnswer);
  };

  let answer = "8";

  const handleTimeout = () => {
    nextGameState(GameState.GameCompleteYouLoseByTimeout);
  };

  return (
    <div className="space-y-4 border border-gray-200 rounded-lg p-4">
      <h3 className="text-lg font-semibold">🎯 Game Actions</h3>
      <p className="text-sm text-muted-foreground">
        Player 2 has sent their answer. Answer is {answer}. What do you want to
        do?
      </p>

      <div className="text-center mt-3">
        <p className="text-xs text-muted-foreground text-center">
          ⏰ If you do nothing, Player 2 will automatically win by timeout
        </p>
        <TimeRemaining numberBlocks={2} onTimeout={handleTimeout} size="lg" />
      </div>

      <div className="p-3 ">
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
              <strong>"Accept Player 2's answer"</strong> → Wait for the timeout
              and lose.
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

      <div className="grid grid-cols-2 gap-4">
        <Button
          onClick={handleAccept}
          className="bg-green-600 hover:bg-green-700"
        >
          ✅ Accept Answer
        </Button>

        <Button
          onClick={handleChallenge}
          variant="destructive"
          className="cursor-pointer bg-orange-600"
        >
          ⚖️ I Disagree - Start Dispute
        </Button>
      </div>
    </div>
  );
}
