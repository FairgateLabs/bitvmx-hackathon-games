import { useQuery } from "@tanstack/react-query";
import { getApiBaseUrl } from "../config/backend";
import { PlayerSymbol } from "@/components/tic-tac-toe/common/tic-tac-toe-board";
import { EnumPlayerRole } from "@/types/game";

interface TicTacToeMove {
  index: number;
  player: EnumPlayerRole;
  playerSymbol: PlayerSymbol;
  gameId: string;
  timestamp: string;
}

interface TicTacToeMovesResponse {
  moves: TicTacToeMove[];
  lastMove?: TicTacToeMove;
}

/**
 * Hook to fetch opponent moves for tic-tac-toe game
 * Polls the backend for new moves every 2 seconds
 */
export function useTicTacToeMoves(gameId?: string) {
  return useQuery({
    queryKey: ["ticTacToeMoves", gameId],
    queryFn: async (): Promise<TicTacToeMove | null> => {
      const baseUrl = getApiBaseUrl();
      const response = await fetch(
        `${baseUrl}/api/tic-tac-toe/${gameId}/moves`
      );

      if (!response.ok) {
        throw new Error("Failed to fetch tic-tac-toe moves");
      }

      const data: TicTacToeMovesResponse = await response.json();
      return data.lastMove || null;
    },
    enabled: !!gameId,
    refetchInterval: 2000, // Poll every 2 seconds
    refetchIntervalInBackground: true,
    staleTime: 1000, // Consider data stale after 1 second
    retry: 3,
    retryDelay: 1000,
  });
}

/**
 * Mock hook for development/testing
 * Returns simulated opponent moves
 */
export function useMockTicTacToeMoves(gameId?: string) {
  const usedMoves = new Set<number>();

  return useQuery({
    queryKey: ["mockTicTacToeMoves", gameId],
    queryFn: async (): Promise<{
      index: number;
      player: EnumPlayerRole;
      playerSymbol: PlayerSymbol;
    }> => {
      // Simulate random opponent moves for testing
      const availableMoves = [0, 1, 2, 3, 4, 5, 6, 7, 8].filter(
        (move) => !usedMoves.has(move)
      );

      if (availableMoves.length === 0) {
        throw new Error("No more available moves");
      }

      const randomIndex =
        availableMoves[Math.floor(Math.random() * availableMoves.length)];

      usedMoves.add(randomIndex);

      return {
        index: randomIndex,
        player: EnumPlayerRole.Player2,
        playerSymbol: PlayerSymbol.O,
      };
    },
    enabled: !!gameId,
    refetchInterval: 5000, // Poll every 5 seconds for demo
    refetchIntervalInBackground: true,
    staleTime: 2000,
    retry: 1,
  });
}
