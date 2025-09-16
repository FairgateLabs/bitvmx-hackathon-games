import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { GameNumbersToAdd } from "@/types/game";
import { useSetupGame, useCurrentGame } from "@/hooks/useGame";

export function SetupGame() {
  const [numbers, setNumbers] = useState<GameNumbersToAdd>({});
  const [inputsDisabled, setInputsDisabled] = useState(false);
  const [isSuccess, setIsSuccess] = useState(false);
  const [isOpen, setIsOpen] = useState(true);
  const { data: game } = useCurrentGame();

  const { mutate: createGame, isPending } = useSetupGame({
    program_id: game?.program_id ?? "",
    number1: numbers.number1 || 0,
    number2: numbers.number2 || 0,
  });

  const generateProgram = () => {
    // Placeholder for the actual generate program logic
    createGame();
    setIsSuccess(true);
    setInputsDisabled(true);
  };

  const handleNumberChange = (key: string, value: string) => {
    const parsedValue = parseInt(value);
    if (parsedValue >= 0) {
      setNumbers({ ...numbers, [key]: parsedValue });
    } else {
      setNumbers({ ...numbers, [key]: 0 });
    }
  };

  return (
    <div className="space-y-4 p-4 rounded-lg border border-gray-200">
      <Collapsible open={isOpen} onOpenChange={setIsOpen}>
        <CollapsibleTrigger asChild>
          <h3 className="font-semibold mb-3 text-gray-800 cursor-pointer hover:text-gray-900">
            🎮 Game Setup
          </h3>
        </CollapsibleTrigger>
        <CollapsibleContent>
          <p className="text-sm text-gray-700 my-4">
            Enter the numbers you wish to use for the game. These will be
            utilized to create the program for your game session.
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
                    handleNumberChange("number1", e.target.value)
                  }
                  placeholder="Eg: 5"
                  disabled={inputsDisabled}
                />
              </div>
              <div className="cursor-pointer">
                <Label htmlFor="number2">Second Number</Label>
                <Input
                  id="number2"
                  type="number"
                  value={numbers.number2 || ""}
                  onChange={(e) =>
                    handleNumberChange("number2", e.target.value)
                  }
                  placeholder="Eg: 3"
                  disabled={inputsDisabled}
                />
              </div>
            </div>

            <Button
              onClick={generateProgram}
              disabled={
                !numbers.number1 ||
                !numbers.number2 ||
                isPending ||
                inputsDisabled
              }
              className="w-full"
            >
              {isPending ? "Generating..." : "🚀 Generate Program"}
            </Button>

            {!isSuccess && (
              <div className="p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
                <h3 className="font-semibold mb-2 text-yellow-800">
                  ⚠️ Choose the numbers to start the program
                </h3>
                <p className="text-sm text-yellow-700">
                  Enter the numbers and click the button to send them to BitVMX
                  for program creation.
                </p>
              </div>
            )}

            {isSuccess && (
              <div className="p-4 bg-green-50 border border-green-200 rounded-lg">
                <h3 className="font-semibold mb-2 text-green-800">
                  ✅ Generation Successful
                </h3>
                <p className="text-sm text-green-700">
                  Program generated successfully with the provided numbers.
                </p>
              </div>
            )}
          </div>
        </CollapsibleContent>
      </Collapsible>
    </div>
  );
}
