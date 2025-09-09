import { useMutation, useQuery } from "@tanstack/react-query";
import { getApiBaseUrl } from "../config/backend";
import { AddNumbersRequest } from "../../../backend/bindings/AddNumbersRequest";
import { AddNumbersGame } from "../../../backend/bindings/AddNumbersGame";
import { AddNumbersGameStatus } from "../../../backend/bindings/AddNumbersGameStatus";

function useGameById(id: string) {
  async function fetchGameById(id: string) {
    const baseUrl = getApiBaseUrl();
    const response = await fetch(`${baseUrl}/api/add-numbers/${id}`);
    if (!response.ok) {
      throw new Error("Failed to fetch game ID");
    }
    const data = await response.json();
    return data.gameId;
  }

  return useQuery({
    queryKey: ["gameId", id],
    queryFn: () => fetchGameById(id),
  });
}

function useCreateGame(data: AddNumbersRequest) {
  return useMutation({
    mutationFn: async () => {
      const baseUrl = getApiBaseUrl();
      const response = await fetch(`${baseUrl}/api/add-numbers`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
      });

      if (!response.ok) {
        throw new Error("Failed to create game");
      }
      return response.json();
    },
  });
}

function useAnswerAddNumber(id: string, number: number) {
  return useMutation({
    mutationFn: async () => {
      const baseUrl = getApiBaseUrl();
      const response = await fetch(`${baseUrl}/api/add-numbers/guess`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ id, number }),
      });
      if (!response.ok) {
        throw new Error("Failed to send number");
      }
      return response.json();
    },
  });
}

function useCurrentGame() {
  async function fetchCurrentGame(): Promise<AddNumbersGame | null> {
    const baseUrl = getApiBaseUrl();
    const response = await fetch(`${baseUrl}/api/add-numbers/current-game-id`);
    if (!response.ok) {
      return null;
    }
    const data = await response.json();
    return data || null;
  }

  const query = useQuery({
    queryKey: ["currentGameId"],
    queryFn: fetchCurrentGame,
  });

  // Return the game status directly from the backend
  const gameStatus = query.data?.status || "WaitingForNumbers";

  return {
    ...query,
    gameStatus,
  };
}

export { useGameById, useCreateGame, useAnswerAddNumber, useCurrentGame };
