import { Trophy } from "lucide-react";

export function TimeoutLoseGame() {
  return (
    <div className="p-4 bg-red-50 border border-red-200 rounded-lg">
      <div className="flex items-center gap-3 mb-3">
        <Trophy className="h-6 w-6 text-red-600" />
        <h3 className="text-lg font-semibold text-red-800">Timeout Defeat!</h3>
      </div>
      <p className="text-sm text-red-700 mb-3">
        You timed out on your answer to the protocol and lost!
      </p>
    </div>
  );
}
