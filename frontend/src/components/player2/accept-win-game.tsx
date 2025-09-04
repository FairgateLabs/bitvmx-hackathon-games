import { Trophy } from "lucide-react";

export function AcceptWinGame() {
  return (
    <div className="p-4 bg-green-50 border border-green-200 rounded-lg">
      <div className="flex items-center gap-3 mb-3">
        <Trophy className="h-6 w-6 text-green-600" />
        <h3 className="text-lg font-semibold text-green-800">
          Victory by Acceptance!
        </h3>
      </div>
      <p className="text-sm text-green-700 mb-3">
        Player 1 accepted your answer, resulting in your victory!
      </p>
      <p className="text-sm text-green-600">
        BitVMX is processing the transaction to finalize this victory.
      </p>
    </div>
  );
}
