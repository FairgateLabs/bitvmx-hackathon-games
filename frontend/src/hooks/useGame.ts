import { useMutation, useQuery } from "@tanstack/react-query";
import { getApiBaseUrl } from "../config/backend";
import { AddNumbersRequest } from "../../../backend/bindings/StartGameRequest";
import { AddNumbersGame } from "../../../backend/bindings/AddNumbersGame";
import { EnumPlayerRole } from "@/types/game";

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

function useCreateGame(data: StartGameRequest) {
  return useMutation({
    mutationFn: async () => {
      const baseUrl = getApiBaseUrl();
      const response = await fetch(`${baseUrl}/api/add-numbers/start-game`, {
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
    if (data.role) {
      data.role = data.role as EnumPlayerRole;
    }

    return data || null;
  }

  return useQuery({
    queryKey: ["currentGameId"],
    queryFn: fetchCurrentGame,
    refetchInterval: 5 * 1000, // every 5 seconds
    refetchIntervalInBackground: true, // keep polling even when tab is not focused
    staleTime: 0, // data becomes stale immediately after fetch
  });
}

export { useGameById, useCreateGame, useAnswerAddNumber, useCurrentGame };
