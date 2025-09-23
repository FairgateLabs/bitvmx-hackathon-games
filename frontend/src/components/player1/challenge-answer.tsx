import { Shield } from "lucide-react";
import { useCurrentGame } from "@/hooks/useGame";
import { BlocksRemaining } from "../ui/block-remaining";
import { EnumPlayerRole } from "@/types/game";

export function ChallengeAnswer() {
  const { data: currentGame } = useCurrentGame();
  const role = currentGame?.role;
  const whoDecidedChallenge =
    role === EnumPlayerRole.Player1 ? "You" : "Player 1";
  const whoIsChallenged = role === EnumPlayerRole.Player1 ? "Player 2" : "Your";

  const handleTimeout = () => {};

  return (
    <div className="p-5 bg-orange-50 border border-orange-200 rounded-lg">
      <h3 className="text-lg font-semibold text-orange-800">
        🚨 Challenge Initiated!
      </h3>

      <div className="text-center mt-3 mb-2">
        <div className="text-xs space-y-4 text-orange-800 pt-3">
          ⏳ Challenge validation is now running on-chain. This may take several
          blocks.
        </div>

        <BlocksRemaining numberBlocks={5} onTimeout={handleTimeout} />
      </div>

      <div className="space-y-4">
        <h4 className="font-semibold  mb-2 flex items-center gap-2">
          <Shield className="h-4 w-4" />
          Challenge Details
        </h4>
        <p className="text-sm  mb-3">
          {whoDecidedChallenge} decided to challenge {whoIsChallenged}&apos;s
          answer. The validation will now run on-chain to determine who is
          right. This step will require multiple on-chain transactions from both
          parties. <br /> All protocol transactions could be seen in the{" "}
          <a
            href="/add-numbers/transactions"
            className="font-bold underline cursor-pointer"
          >
            transaction&apos;s tab
          </a>
        </p>

        <h4 className="font-semibold mb-2">💰 Winner&apos;s Reward</h4>
        <p className="text-sm ">
          The winner will receive the total amount bet after the challenge is
          resolved.
        </p>
      </div>
    </div>
  );
}
