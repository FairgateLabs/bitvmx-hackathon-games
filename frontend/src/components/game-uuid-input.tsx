import { useState } from "react";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Check, Copy } from "lucide-react";
import { CopyButton } from "@/components/ui/copy-button";
import { useProgramMutation } from "@/hooks/usePrograms";

interface GameUUIDInputProps {
  isExpanded?: boolean;
}

export function GameUUIDInput({ isExpanded = true }: GameUUIDInputProps) {
  const [gameUUID, setGameUUID] = useState("");
  const [isExpandedLocal, setIsExpandedLocal] = useState(isExpanded);
  const [isValid, setIsValid] = useState(false);
  const { mutate: saveProgram } = useProgramMutation();

  const handleUUIDChange = (value: string) => {
    setGameUUID(value);
    // Basic UUID validation (8-4-4-4-12 format)
    const uuidRegex =
      /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;
    setIsValid(uuidRegex.test(value));
  };

  const handleSubmit = () => {
    if (isValid && gameUUID) {
      saveProgram(gameUUID);
    }
  };

  return (
    <div className="p-4 bg-gray-50 border border-gray-200 rounded-lg">
      <h3
        className="font-semibold mb-3 text-gray-800 cursor-pointer"
        onClick={() => setIsExpandedLocal(!isExpandedLocal)}
      >
        🎯 Enter Game UUID {isExpandedLocal ? "▲" : "▼"}
      </h3>
      {isExpandedLocal && (
        <>
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
                <p className="text-xs text-gray-600 mt-1">
                  Please enter a valid UUID format
                </p>
              )}
            </div>

            <p className="text-sm text-gray-800 mb-3">
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
                  placeholder="e.g., 5"
                  className="mt-1 text-sm"
                />
              </div>
              <div>
                <Label htmlFor="number2" className="text-gray-800 text-xs">
                  Second Number:
                </Label>
                <Input
                  id="number2"
                  type="number"
                  placeholder="e.g., 3"
                  className="mt-1 text-sm"
                />
              </div>
            </div>

            <Button
              onClick={handleSubmit}
              disabled={!isValid || !gameUUID}
              className="w-full bg-gray-600 hover:bg-gray-700"
            >
              <Check className="h-4 w-4 mr-2" />
              Join Game
            </Button>
          </div>
        </>
      )}
    </div>
  );
}
