import { useState, useEffect } from "react";
import { CopyButton } from "@/components/ui/copy-button";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";

interface GameUUIDGeneratorProps {
  isExpanded?: boolean;
  onUUIDChange?: (uuid: string) => void;
}

export function GameUUIDGenerator({
  isExpanded = true,
  onUUIDChange,
}: GameUUIDGeneratorProps) {
  const [gameUUID, setGameUUID] = useState<string>("");
  const [isOpen, setIsOpen] = useState(isExpanded);

  const generateUUID = () => {
    const uuid = crypto.randomUUID();
    setGameUUID(uuid);
  };

  useEffect(() => {
    if (!gameUUID) {
      generateUUID();
    }
  }, []);

  useEffect(() => {
    if (gameUUID && onUUIDChange) {
      onUUIDChange(gameUUID);
    }
  }, [gameUUID, onUUIDChange]);

  return (
    <div>
      <Collapsible open={isOpen} onOpenChange={setIsOpen}>
        <CollapsibleTrigger asChild>
          <h3 className="font-semibold mb-3 text-gray-800 cursor-pointer hover:text-gray-900">
            🎮 Game UUID
          </h3>
        </CollapsibleTrigger>
        <CollapsibleContent>
          <p className="text-sm text-gray-700 mb-4">
            Share this unique game identifier with Player 2 so they can join
            your game.
          </p>

          <div className="space-y-3">
            <div className="flex items-center justify-between">
              <div className="flex-1">
                <p className="text-sm text-gray-800 mb-1">Game UUID:</p>
                <p className="font-mono text-sm bg-gray-100 p-3 rounded break-all">
                  {gameUUID || "Generating..."}
                </p>
              </div>
              <div className="flex gap-2 ml-3 mt-5">
                <CopyButton text={gameUUID} size="sm" variant="outline" />
              </div>
            </div>
          </div>
        </CollapsibleContent>
      </Collapsible>
    </div>
  );
}
