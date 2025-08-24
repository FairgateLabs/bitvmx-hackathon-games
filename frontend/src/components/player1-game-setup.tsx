import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

interface GameNumbersToAdd {
  number1?: number;
  number2?: number;
}

interface Player1GameSetupProps {
  numbers: GameNumbersToAdd;
  setNumbers: (numbers: GameNumbersToAdd) => void;
  generateProgram: () => void;
  gameId: string;
  gameState: string;
  isLoading: boolean;
}

export function Player1GameSetup({
  numbers,
  setNumbers,
  generateProgram,
  gameId,
  gameState,
  isLoading,
}: Player1GameSetupProps) {
  return (
    <div className="space-y-4">
      <h3 className="text-lg font-semibold">🎲 Game Setup</h3>

      {!gameId ? (
        <div className="space-y-4">
          <div className="grid grid-cols-2 gap-4">
            <div className="cursor-pointer">
              <Label htmlFor="number1">First Number</Label>
              <Input
                id="number1"
                type="number"
                value={numbers.number1 || ""}
                onChange={(e) =>
                  setNumbers({ ...numbers, number1: parseInt(e.target.value) })
                }
                placeholder="Eg: 5"
              />
            </div>
            <div className="cursor-pointer">
              <Label htmlFor="number2">Second Number</Label>
              <Input
                id="number2"
                type="number"
                value={numbers.number2 || ""}
                onChange={(e) =>
                  setNumbers({ ...numbers, number2: parseInt(e.target.value) })
                }
                placeholder="Eg: 3"
              />
            </div>
          </div>

          <Button
            onClick={generateProgram}
            disabled={!numbers.number1 || !numbers.number2 || isLoading}
            className="w-full cursor-pointer"
          >
            {isLoading ? "Generating..." : "🚀 Generate Program"}
          </Button>
        </div>
      ) : (
        <div className="p-4 bg-green-50 border border-green-200 rounded-md">
          <h4 className="font-semibold text-green-800 mb-2">
            ✅ Program Generated
          </h4>
          <p className="text-sm text-green-700 mb-2">Game ID:</p>
          <p className="font-mono text-xs bg-green-100 p-2 rounded break-all">
            {gameId}
          </p>
          <p className="text-sm text-green-700 mt-2">
            Share this ID with Player 2 to join the game.
          </p>
        </div>
      )}

      {gameState === "waiting_response" && (
        <div className="p-4 bg-blue-50 border border-blue-200 rounded-md">
          <h4 className="font-semibold text-blue-800">
            ⏳ Waiting for Player 2&apos;s response...
          </h4>
          <p className="text-sm text-blue-700">
            Player 2 is calculating the sum of {numbers.number1} +{" "}
            {numbers.number2}
          </p>
        </div>
      )}
    </div>
  );
}
