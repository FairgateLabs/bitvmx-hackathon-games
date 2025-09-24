import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { EnumPlayerRole, GameNumbersToAdd } from "@/types/game";
import { useSetupGame, useCurrentGame } from "@/hooks/useGame";
import { useQueryClient } from "@tanstack/react-query";

export function SetupGame() {
  const [numbers, setNumbers] = useState<GameNumbersToAdd>({});
  const [isOpen, setIsOpen] = useState(true);
  const { data: game } = useCurrentGame();
  const queryClient = useQueryClient();
  const { mutate: createGame, isPending } = useSetupGame({
    program_id: game?.program_id ?? "",
    number1: numbers.number1 || 0,
    number2: numbers.number2 || 0,
  });

  useEffect(() => {
    queryClient.invalidateQueries({ queryKey: ["currentGame"] });
  }, [isPending]);

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
                  disabled={isPending}
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
                  disabled={isPending}
                />
              </div>
            </div>

            <Button
              onClick={() => createGame()}
              disabled={
                !numbers.number1 || !numbers.number2 || isPending || isPending
              }
              className="w-full"
            >
              {isPending ? "⏳ Generating Program..." : "🚀 Generate Program"}
            </Button>

            {!isPending && (
              <div className="p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
                <h3 className="font-semibold mb-2 text-yellow-800">
                  {game?.role === EnumPlayerRole.Player1
                    ? "⚠️  Choose the numbers to setup the game"
                    : "⚠️  Enter the numbers to setup the same game"}
                </h3>
                <p className="text-sm text-yellow-700">
                  Remember, both players must enter the same numbers. <br />
                  {game?.role === EnumPlayerRole.Player1 && (
                    <>
                      Ensure you share the numbers with Player 2 so both players
                      can participate in the same game.
                    </>
                  )}
                  {game?.role === EnumPlayerRole.Player2 && (
                    <>
                      Enter the numbers that Player 1 choose to setup the same
                      game. Otherwise, you won&apos;t be able to participate in
                      the same game.
                    </>
                  )}
                </p>
              </div>
            )}
          </div>
        </CollapsibleContent>
      </Collapsible>
    </div>
  );
}
