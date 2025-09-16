import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { CopyButton } from "@/components/ui/copy-button";
import { useNetworkQuery } from "@/hooks/useNetwork";
import { NetworkType } from "@/types/network";

export function SetupGame() {
  const [isLoading, setIsLoading] = useState(false);
  const [inputsDisabled, setInputsDisabled] = useState(false);
  const [isOpen, setIsOpen] = useState(true);
  const { data: network } = useNetworkQuery();

  const generateProgram = () => {
    // Placeholder for the actual generate program logic
    setIsLoading(true);
    setTimeout(() => {
      setIsLoading(false);
      setInputsDisabled(true);
    }, 2000);
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
  }, [gameUUID]);

  const amountToBet = network && network === NetworkType.Regtest ? 1 : 0.0001;

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

            <div className="space-y-4">
              <Button
                onClick={generateProgram}
                disabled={isLoading || inputsDisabled}
                className="w-full"
              >
                {isLoading ? "Generating..." : "🚀 Generate Program"}
              </Button>
            </div>
          </CollapsibleContent>
        </div>
      </Collapsible>
    </div>
  );
}
