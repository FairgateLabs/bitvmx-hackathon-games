import { Trophy } from "lucide-react";

export function ChallengeWinGame() {
  return (
    <div className="p-4 bg-green-50 border border-green-200 rounded-lg">
      <div className="flex items-center gap-3 mb-3">
        <Trophy className="h-6 w-6 text-green-600" />
        <h3 className="text-lg font-semibold text-green-800">
          Challenge Successful!
        </h3>
      </div>
      <p className="text-sm text-green-700 mb-3">
        Bit You challenged Player 2&apos;s answer and the BitVMX protocol has
        validated on-chain that you were correct!
      </p>
    </div>
  );
}
