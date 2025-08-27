import { useQuery } from "@tanstack/react-query";
import { GameState } from "@/types/gameState";

async function fetchGameState(): Promise<GameState> {
  const response = await fetch("/api/game-state");
  const data = await response.json();
  return data.stateOfTheGame;
}

export function useGameState() {
  return useQuery<GameState>({
    queryKey: ["gameState"],
    queryFn: fetchGameState,
    initialData: GameState.Setup,
  });
}
