import { GameState } from "@/types/game";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

function useGameState() {
  return useQuery<GameState>({
    queryKey: ["gameState"],
    queryFn: async () => {
      const queryClient = useQueryClient();
      const current = queryClient.getQueryData<GameState>(["gameState"]);
      return current || GameState.ChooseGame;
    },
    initialData: GameState.ChooseGame,
  });
}

function useNextGameState() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (gameState: GameState) => {
      queryClient.setQueryData(["gameState"], gameState);
      return gameState;
    },
  });
}

export { useGameState, useNextGameState };
