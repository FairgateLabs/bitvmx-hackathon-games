import { Trophy } from "lucide-react";

export function ChallengeLoseGame() {
  return (
    <div className="p-4 bg-red-50 border border-red-200 rounded-lg">
      <div className="flex items-center gap-3 mb-3">
        <Trophy className="h-6 w-6 text-red-600" />
        <h3 className="text-lg font-semibold text-red-800">Challenge Lost!</h3>
      </div>
      <p className="text-sm text-red-700 mb-3">
        Player 1 challenged your answer and the BitVMX protocol has validated
        on-chain that they were incorrect!
      </p>
    </div>
  );
}
