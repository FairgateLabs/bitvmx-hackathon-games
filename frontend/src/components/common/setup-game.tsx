import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { CopyButton } from "../ui/copy-button";
import { GameNumbersToAdd } from "@/types/gameState";
import { useNextGameState } from "@/hooks/useGameState";
import { useNetwork } from "@/hooks/useNetwork";
import { NetworkType } from "@/types/network";

export function SetupGame() {
  const [numbers, setNumbers] = useState<GameNumbersToAdd>({});
  const [isLoading, setIsLoading] = useState(false);
  const [inputsDisabled, setInputsDisabled] = useState(false);
  const [successMessage, setSuccessMessage] = useState("");
  const [isOpen, setIsOpen] = useState(true);
  const { mutate: nextGameState } = useNextGameState();
  const { data: network } = useNetwork();

  const generateProgram = () => {
    // Placeholder for the actual generate program logic
    setIsLoading(true);
    setTimeout(() => {
      setIsLoading(false);
      setInputsDisabled(true);
      setSuccessMessage(
        "Program generated successfully with the provided numbers."
      );
      console.log("Program generated with numbers:", numbers);
    }, 2000);
    nextGameState(null);
  };

  const handleNumberChange = (key: string, value: string) => {
    const parsedValue = parseInt(value);
    if (parsedValue >= 0) {
      setNumbers({ ...numbers, [key]: parsedValue });
    } else {
      setNumbers({ ...numbers, [key]: 0 });
    }
  };

  const [gameUUID, setGameUUID] = useState<string>("");

  const generateUUID = () => {
    const uuid = crypto.randomUUID();
    setGameUUID(uuid);
  };

  useEffect(() => {
    if (!gameUUID) {
      generateUUID();
    }
  }, []);

  let amountToBet = network && network === NetworkType.Regtest ? 1 : 0.0001;

  return (
    <div className="space-y-4 p-4 rounded-lg border border-gray-200">
      <Collapsible open={isOpen} onOpenChange={setIsOpen}>
        <div>
          <CollapsibleTrigger asChild>
            <h3 className="font-semibold mb-3 text-gray-800 cursor-pointer hover:text-gray-900">
              🎮 Game Setup
            </h3>
          </CollapsibleTrigger>
          <CollapsibleContent>
            <p className="text-sm text-gray-700 mb-4">
              Share this unique game identifier with Player 2 so they can join
              your game.
            </p>

            <div className="space-y-3 flex gap-8">
              <div className="flex items-center justify-between">
                <div className="flex-1">
                  <p className="text-sm text-gray-700 mb-1">Game UUID:</p>
                  <p className="font-mono text-sm bg-gray-100 p-3 rounded break-all">
                    {gameUUID || "Generating..."}
                  </p>
                </div>
                <div className="flex gap-2 ml-3 mt-5">
                  <CopyButton text={gameUUID} size="sm" variant="outline" />
                </div>
              </div>
              <div className="space-y-3 mt-4">
                <div className="flex items-center justify-between gap-2 pt-4">
                  <p className="text-sm text-gray-700">Amount to Bet:</p>
                  <p className="font-mono text-sm">{amountToBet} BTC</p>
                </div>
              </div>
            </div>

            <p className="text-sm text-gray-700 my-4">
              Please enter the numbers you wish to use for the game. These will
              be utilized to create the program for your game session.
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
                  isLoading ||
                  inputsDisabled
                }
                className="w-full"
              >
                {isLoading ? "Generating..." : "🚀 Generate Program"}
              </Button>

              {!successMessage && (
                <div className="p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
                  <h3 className="font-semibold mb-2 text-yellow-800">
                    ⚠️ Choose the numbers to start the program
                  </h3>
                  <p className="text-sm text-yellow-700">
                    Please enter the numbers and click the button to send them
                    to BitVMX for program creation.
                  </p>
                </div>
              )}

              {successMessage && (
                <div className="p-4 bg-green-50 border border-green-200 rounded-lg">
                  <h3 className="font-semibold mb-2 text-green-800">
                    ✅ UUID Generation Successful
                  </h3>
                  <p className="text-sm text-green-700">{successMessage}</p>
                </div>
              )}
            </div>
          </CollapsibleContent>
        </div>
      </Collapsible>
    </div>
  );
}
