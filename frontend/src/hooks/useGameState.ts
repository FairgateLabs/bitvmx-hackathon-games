import { GameState } from "@/types/gameState";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

const gameStateOrder: GameState[] = [
  GameState.ChooseGame,
  GameState.SetupNetwork,
  GameState.ChooseRole,
  // GameState.SetupFunding,
  GameState.SetupConnection,
  GameState.SetupProgram,
  GameState.StartGame,
  GameState.ChooseAction,
  GameState.ChallengeAnswer,
  GameState.GameCompleteYouLose,
  GameState.GameCompleteYouWin,
  GameState.TransferFunds,
];

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
    mutationFn: async (gameState: GameState | null = null) => {
      if (gameState) {
        return gameState;
      }

      // read current state from cache
      const current =
        queryClient.getQueryData<GameState>(["gameState"]) ||
        GameState.ChooseGame;
      console.log("current", current);

      const idx = gameStateOrder.indexOf(current);
      let next =
        idx >= 0 && idx < gameStateOrder.length - 1
          ? gameStateOrder[idx + 1]
          : GameState.GameCompleteYouWin;
      console.log("next", next);
      return next;
    },
    onSuccess: (nextState) => {
      // update cache so useGameState sees new value
      queryClient.setQueryData(["gameState"], nextState);
    },
  });
}

export { useGameState, useNextGameState };
