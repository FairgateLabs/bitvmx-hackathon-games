import { Trophy } from "lucide-react";
import { TimeRemaining } from "../ui/time-remaining";

export function AcceptLoseGame() {
  const handleTimeout = () => {
    console.log("handleTimeout");
  };

  return (
    <div className="p-4 bg-red-50 border border-red-200 rounded-lg">
      <div className="flex items-center gap-3 mb-3">
        <Trophy className="h-6 w-6 text-red-600" />
        <h3 className="text-lg font-semibold text-red-800">You Lost!</h3>
      </div>

      <p className="text-sm text-red-600">
        You accepted Player 2&apos;s answer, resulting in their victory.
      </p>
      <p className="text-sm text-red-600">
        BitVMX is processing the transaction to finalize this acceptance.
      </p>

      <TimeRemaining numberBlocks={5} size="lg" onTimeout={handleTimeout} />
    </div>
  );
}
