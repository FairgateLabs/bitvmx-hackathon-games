import { useState, useEffect } from "react";
import { CopyButton } from "@/components/ui/copy-button";

interface GameUUIDGeneratorProps {
  isExpanded?: boolean;
}

export function GameUUIDGenerator({
  isExpanded = true,
}: GameUUIDGeneratorProps) {
  const [gameUUID, setGameUUID] = useState<string>("");
  const [isExpandedLocal, setIsExpandedLocal] = useState(isExpanded);

  const generateUUID = () => {
    const uuid = crypto.randomUUID();
    setGameUUID(uuid);
  };

  useEffect(() => {
    if (!gameUUID) {
      generateUUID();
    }
  }, []);

  return (
    <div className="p-4 bg-purple-50 border border-purple-200 rounded-lg">
      <h3
        className="font-semibold mb-3 text-purple-800 cursor-pointer"
        onClick={() => setIsExpandedLocal(!isExpandedLocal)}
      >
        🎮 Game UUID {isExpandedLocal ? "▲" : "▼"}
      </h3>
      {isExpandedLocal && (
        <>
          <p className="text-sm text-purple-700 mb-4">
            Share this unique game identifier with Player 2 so they can join
            your game.
          </p>

          <div className="space-y-3">
            <div className="flex items-center justify-between">
              <div className="flex-1">
                <p className="text-sm text-purple-800 mb-1">Game UUID:</p>
                <p className="font-mono text-sm bg-purple-100 p-3 rounded break-all">
                  {gameUUID || "Generating..."}
                </p>
              </div>
              <div className="flex gap-2 ml-3 mt-5">
                <CopyButton text={gameUUID} size="sm" variant="outline" />
              </div>
            </div>
          </div>
        </>
      )}
    </div>
  );
}
