import { AlertTriangle, Shield, Coins, Link } from "lucide-react";
import { GameRole } from "../common/game-role-selector";
import { useGameRole } from "@/hooks/useGameRole";

export function ChallengeAnswer() {
  const { data: role } = useGameRole();
  let whoDecidedChallenge = role === GameRole.Player1 ? "You" : "Player 1";
  let whoIsChallenged = role === GameRole.Player1 ? "Player 2" : "Your";

  return (
    <div className="p-5 bg-orange-50 border border-orange-200 rounded-lg">
      <div className="flex items-center gap-3">
        <h3 className="text-lg font-semibold text-orange-800">
          🚨 Challenge Initiated!
        </h3>
      </div>

      <div className="text-xs space-y-4 text-orange-800 pt-3 pb-5">
        ⏳ Challenge validation is now running on-chain. This may take several
        blocks.
      </div>

      <div className="space-y-4">
        <h4 className="font-semibold  mb-2 flex items-center gap-2">
          <Shield className="h-4 w-4" />
          Challenge Details
        </h4>
        <p className="text-sm  mb-3">
          {whoDecidedChallenge} decided to challenge {whoIsChallenged}'s answer.
          The validation will now run on-chain to determine who is right. This
          step will require multiple on-chain transactions from both parties.{" "}
          <br /> All protocol transactions could be seen in the{" "}
          <a
            href="/add-numbers/transactions"
            className="font-bold underline cursor-pointer"
          >
            transaction's tab
          </a>
        </p>

        <h4 className="font-semibold mb-2">💰 Winner's Reward</h4>
        <p className="text-sm ">
          The winner will receive the total amount bet after the challenge is
          resolved.
        </p>
      </div>
    </div>
  );
}
