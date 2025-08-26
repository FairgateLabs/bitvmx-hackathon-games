import { useState } from "react";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Check, Copy } from "lucide-react";

interface GameUUIDInputProps {
  onUUIDEntered: (uuid: string) => void;
  isExpanded?: boolean;
}

export function GameUUIDInput({
  onUUIDEntered,
  isExpanded = true,
}: GameUUIDInputProps) {
  const [gameUUID, setGameUUID] = useState("");
  const [isExpandedLocal, setIsExpandedLocal] = useState(isExpanded);
  const [isValid, setIsValid] = useState(false);

  const handleUUIDChange = (value: string) => {
    setGameUUID(value);
    // Basic UUID validation (8-4-4-4-12 format)
    const uuidRegex =
      /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;
    setIsValid(uuidRegex.test(value));
  };

  const handleSubmit = () => {
    if (isValid && gameUUID) {
      onUUIDEntered(gameUUID);
    }
  };

  const pasteFromClipboard = async () => {
    try {
      const text = await navigator.clipboard.readText();
      handleUUIDChange(text);
    } catch (error) {
      console.error("Failed to read from clipboard:", error);
    }
  };

  return (
    <div className="p-4 bg-orange-50 border border-orange-200 rounded-lg">
      <h3
        className="font-semibold mb-3 text-orange-800 cursor-pointer"
        onClick={() => setIsExpandedLocal(!isExpandedLocal)}
      >
        🎯 Enter Game UUID {isExpandedLocal ? "▲" : "▼"}
      </h3>
      {isExpandedLocal && (
        <>
          <p className="text-sm text-orange-700 mb-4">
            Enter the game UUID provided by Player 1 to join their game session.
          </p>

          <div className="space-y-3">
            <div>
              <Label htmlFor="gameUUID" className="text-orange-800">
                Game UUID:
              </Label>
              <div className="flex gap-2 mt-1">
                <Input
                  id="gameUUID"
                  value={gameUUID}
                  onChange={(e) => handleUUIDChange(e.target.value)}
                  placeholder="e.g., 123e4567-e89b-12d3-a456-426614174000"
                  className={`flex-1 ${
                    gameUUID && !isValid ? "border-red-300" : ""
                  }`}
                />
                <Button
                  size="sm"
                  variant="outline"
                  onClick={pasteFromClipboard}
                  className="border-orange-300 text-orange-700 hover:bg-orange-100"
                >
                  <Copy className="h-4 w-4" />
                </Button>
              </div>
              {gameUUID && !isValid && (
                <p className="text-xs text-red-600 mt-1">
                  Please enter a valid UUID format
                </p>
              )}
            </div>

            <Button
              onClick={handleSubmit}
              disabled={!isValid || !gameUUID}
              className="w-full bg-orange-600 hover:bg-orange-700"
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
