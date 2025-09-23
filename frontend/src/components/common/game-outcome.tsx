import { Trophy, Clock, Sword } from "lucide-react";
import { useCurrentGame } from "@/hooks/useGame";

export function GameOutcome() {
  const { data: currentGame } = useCurrentGame();
  const isWin =
    typeof currentGame?.status === "object" &&
    currentGame?.status.GameComplete.outcome === "Win";
  const isChallenge =
    typeof currentGame?.status === "object" &&
    currentGame?.status.GameComplete.reason === "Challenge";
  const isTimeout =
    typeof currentGame?.status === "object" &&
    currentGame?.status.GameComplete.reason === "Timeout";

  // Determine the styling based on win/lose
  const containerClass = isWin
    ? "p-4 bg-green-50 border border-green-200 rounded-lg"
    : "p-4 bg-red-50 border border-red-200 rounded-lg";

  const iconClass = isWin ? "h-6 w-6 text-green-600" : "h-6 w-6 text-red-600";

  const titleClass = isWin
    ? "text-lg font-semibold text-green-800"
    : "text-lg font-semibold text-red-800";

  const textClass = isWin ? "text-sm text-green-700" : "text-sm text-red-700";

  // Get the appropriate icon based on reason
  const getIcon = () => {
    if (isChallenge) {
      return <Sword className={iconClass} />;
    } else if (isTimeout) {
      return <Clock className={iconClass} />;
    }
    return <Trophy className={iconClass} />;
  };

  // Generate title based on outcome, reason, and player
  const getTitle = () => {
    if (isWin) {
      if (isChallenge) {
        return "Challenge Successful!";
      } else if (isTimeout) {
        return "Timeout Victory!";
      }
      return "You Won!";
    } else {
      if (isChallenge) {
        return "Challenge Failed!";
      } else if (isTimeout) {
        return "Timeout Loss!";
      }
      return "You Lost!";
    }
  };

  // Generate description based on outcome, reason, and player
  const getDescription = () => {
    const otherPlayer =
      currentGame?.role === "Player1" ? "Player 2" : "Player 1";

    if (isWin) {
      if (isChallenge) {
        return `You challenged ${otherPlayer}'s answer and the BitVMX protocol has validated on-chain that you were correct!`;
      } else if (isTimeout) {
        return `${otherPlayer} timed out on their answer to the protocol and lost!`;
      }
      return `Congratulations! You have won the game!`;
    } else {
      if (isChallenge) {
        return `Your challenge was unsuccessful. ${otherPlayer} provided the correct answer.`;
      } else if (isTimeout) {
        return `You timed out on your answer to the protocol and lost.`;
      }
      return `Unfortunately, you have lost the game.`;
    }
  };

  return (
    <div className={containerClass}>
      <div className="flex items-center gap-3 mb-3">
        {getIcon()}
        <h3 className={titleClass}>{getTitle()}</h3>
      </div>
      <p className={`${textClass} mb-3`}>{getDescription()}</p>
      {isTimeout && (
        <p className={`${textClass} text-xs`}>
          The game ended due to a timeout in the protocol.
        </p>
      )}
      {isChallenge && (
        <p className={`${textClass} text-xs`}>
          The game outcome was determined through an on-chain challenge.
        </p>
      )}
    </div>
  );
}
