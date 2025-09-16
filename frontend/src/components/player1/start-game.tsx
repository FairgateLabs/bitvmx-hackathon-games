import { Button } from "@/components/ui/button";
import { Play } from "lucide-react";
import { useCurrentGame, useStartGame } from "@/hooks/useGame";
import { useQueryClient } from "@tanstack/react-query";
import { useEffect } from "react";

export function StartGame() {
  const { mutate: startGame, isPending } = useStartGame();
  const { data: game } = useCurrentGame();
  const queryClient = useQueryClient();

  const handleStartGame = () => {
    startGame({ program_id: game?.program_id ?? "" });
  };

  useEffect(() => {
    queryClient.invalidateQueries({ queryKey: ["currentGameId"] });
  }, [isPending]);

  return (
    <div className="p-4  border border-gray-200 rounded-lg">
      <div className="flex items-center gap-3 mb-3">
        <h3 className="text-lg font-semibold text-gray-800">
          🚀 Ready to Start
        </h3>
      </div>
      <p className="text-sm text-gray-700 mb-4">
        Everything is complete! The program is finally set up and ready to go.
      </p>

      <div className="p-3 bg-gray-50 rounded-lg mb-4">
        <h4 className="font-semibold text-gray-800 mb-2">
          ⚠️ Game Start Information
        </h4>
        <ul className="text-sm text-gray-700 space-y-1">
          <li>
            The first transaction is submitted on-chain by you when you click
            the Start Game button
          </li>
          <li>
            The game starts and the timer begins for Player 2. Player 2 has 1
            minute to respond.
          </li>
          <li>
            If Player 2 does not respond in time, you automatically win due to
            the transaction timeout
          </li>
        </ul>
      </div>
      <Button
        onClick={handleStartGame}
        className="w-full bg-gray-600 hover:bg-gray-700"
      >
        <Play className="h-4 w-4 mr-2" />
        {isPending ? "Starting Game..." : "Start Game"}
      </Button>
    </div>
  );
}
