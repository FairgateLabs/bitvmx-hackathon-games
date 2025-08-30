import { Trophy } from "lucide-react";

export function YouLost() {
  return (
    <div className="p-4 bg-red-50 border border-red-200 rounded-lg">
      <div className="flex items-center gap-3 mb-3">
        <Trophy className="h-6 w-6 text-red-600" />
        <h3 className="text-lg font-semibold text-red-800">😔 You Lost!</h3>
      </div>
      <p className="text-xs text-red-600">
        The game has ended. Player 2 wins because you accepted their answer.
      </p>
    </div>
  );
}
