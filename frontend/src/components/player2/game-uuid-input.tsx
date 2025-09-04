import { useState } from "react";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Check } from "lucide-react";
import { CopyButton } from "@/components/ui/copy-button";
import { useProgramMutation } from "@/hooks/usePrograms";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { GameNumbersToAdd } from "@/types/game";

interface GameUUIDInputProps {
  isExpanded?: boolean;
}

export function GameUUIDInput({ isExpanded = true }: GameUUIDInputProps) {
  const [gameUUID, setGameUUID] = useState("");
  const [isValid, setIsValid] = useState(false);
  const [isOpen, setIsOpen] = useState(isExpanded);
  const [numbers, setNumbers] = useState<GameNumbersToAdd>({});
  const [isSuccess, setIsSuccess] = useState(false);
  const { mutate: saveProgram } = useProgramMutation();

  const handleUUIDChange = (value: string) => {
    setGameUUID(value);
    // Basic UUID validation (8-4-4-4-12 format)
    const uuidRegex =
      /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;
    setIsValid(uuidRegex.test(value));
  };

  const handleNumberChange = (key: string, value: string) => {
    const parsedValue = parseInt(value);
    if (parsedValue >= 0) {
      setNumbers({ ...numbers, [key]: parsedValue });
    } else {
      setNumbers({ ...numbers, [key]: 0 });
    }
  };

  const handleSubmit = () => {
    if (
      isValid &&
      gameUUID &&
      numbers.number1 !== undefined &&
      numbers.number2 !== undefined
    ) {
      saveProgram(gameUUID);
      setIsSuccess(true);
    }
  };

  return (
    <div className="p-4 bg-white border border-gray-200 rounded-lg">
      <Collapsible open={isOpen} onOpenChange={setIsOpen}>
        <CollapsibleTrigger asChild>
          <h3 className="font-semibold mb-3 text-gray-800 cursor-pointer hover:text-gray-900">
            🎯 Enter Game UUID and Numbers
          </h3>
        </CollapsibleTrigger>
        <CollapsibleContent>
          <p className="text-sm text-gray-700 mb-4">
            Enter the game UUID provided by Player 1 to join their game session.
          </p>

          <div className="space-y-3">
            <div>
              <Label htmlFor="gameUUID" className="text-gray-800">
                Game UUID:
              </Label>
              <div className="flex gap-2 mt-1">
                <Input
                  id="gameUUID"
                  value={gameUUID}
                  onChange={(e) => handleUUIDChange(e.target.value)}
                  placeholder="e.g., 123e4567-e89b-12d3-a456-426614174000"
                  className={`flex-1 ${
                    gameUUID && !isValid ? "border-gray-300" : ""
                  }`}
                />

                <CopyButton text="" size="sm" variant="outline" />
              </div>
              {gameUUID && !isValid && (
                <p className="text-xs text-red-600 mt-1">
                  Please enter a valid UUID format
                </p>
              )}
            </div>

            <p className="text-sm text-gray-700 mb-3">
              <strong>Enter the numbers chosen by Player 1:</strong>
            </p>
            <div className="grid grid-cols-2 gap-3">
              <div>
                <Label htmlFor="number1" className="text-gray-800 text-xs">
                  First Number:
                </Label>
                <Input
                  id="number1"
                  type="number"
                  value={numbers.number1 || ""}
                  placeholder="e.g., 5"
                  className="mt-1 text-sm"
                  required
                  min="0"
                  onChange={(e) =>
                    handleNumberChange("number1", e.target.value)
                  }
                />
              </div>
              <div>
                <Label htmlFor="number2" className="text-gray-800 text-xs">
                  Second Number:
                </Label>
                <Input
                  id="number2"
                  type="number"
                  value={numbers.number2 || ""}
                  placeholder="e.g., 3"
                  className="mt-1 text-sm"
                  min="0"
                  onChange={(e) =>
                    handleNumberChange("number2", e.target.value)
                  }
                />
              </div>
            </div>

            <Button
              onClick={handleSubmit}
              disabled={
                !isValid ||
                !gameUUID ||
                numbers.number1 === undefined ||
                numbers.number2 === undefined
              }
              className="w-full bg-gray-600 hover:bg-gray-700"
            >
              <Check className="h-4 w-4 mr-2" />
              Join Game
            </Button>

            {!isSuccess && (
              <div className="p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
                <h3 className="font-semibold mb-2 text-yellow-800">
                  ⚠️ Complete all inputs to start the game
                </h3>
                <p className="text-sm text-yellow-700">
                  Please enter the game UUID and both numbers to proceed.
                </p>
              </div>
            )}

            {isSuccess && (
              <div className="p-4 bg-green-50 border border-green-200 rounded-lg">
                <h3 className="font-semibold mb-2 text-green-800">
                  ✅ Success
                </h3>
                <p className="text-sm text-green-700">
                  Game joined successfully with the provided UUID and numbers.
                </p>
              </div>
            )}
          </div>
        </CollapsibleContent>
      </Collapsible>
    </div>
  );
}
