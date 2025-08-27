import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { GameUUIDGenerator } from "./game-uuid-generator";

interface GameNumbersToAdd {
  number1?: number;
  number2?: number;
}

export function Player1GameSetup() {
  const [numbers, setNumbers] = useState<GameNumbersToAdd>({});
  const [isLoading, setIsLoading] = useState(false);

  const generateProgram = () => {
    // Placeholder for the actual generate program logic
    setIsLoading(true);
    setTimeout(() => {
      setIsLoading(false);
      console.log("Program generated with numbers:", numbers);
    }, 2000);
  };

  return (
    <div className="space-y-4 p-4 rounded-lg bg-gray-100">
      <GameUUIDGenerator />
      <h3 className="text-lg font-semibold">🎲 Game Setup</h3>
      <p className="text-sm text-gray-700 mb-4">
        Enter the numbers you want to add for the game. These numbers will be
        used to generate the program for the game session.
      </p>

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
    </div>
  );
}
