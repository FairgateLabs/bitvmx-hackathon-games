import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { getApiBaseUrl } from "../config/backend";
import { AddNumbersGame } from "../../../backend/bindings/AddNumbersGame";
import { StartGameRequest } from "../../../backend/bindings/StartGameRequest";
import { SetupGameRequest } from "../../../backend/bindings/SetupGameRequest";
import { SubmitSumRequest } from "../../../backend/bindings/SubmitSumRequest";
import { EnumPlayerRole } from "@/types/game";

function useGameById(id: string) {
  async function fetchGameById() {
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
    queryFn: () => fetchGameById(),
  });
}

function useStartGame() {
  return useMutation({
    mutationFn: async (data: StartGameRequest) => {
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

function useSetupGame(data: SetupGameRequest) {
  return useMutation({
    mutationFn: async () => {
      const baseUrl = getApiBaseUrl();
      await fetch(`${baseUrl}/api/add-numbers/setup-game`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
      });
    },
  });
}

async function submitSum(data: SubmitSumRequest) {
  const baseUrl = getApiBaseUrl();
  const response = await fetch(`${baseUrl}/api/add-numbers/submit-sum`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  });
  if (!response.ok) {
    throw new Error("Failed to send number");
  }
  return response.json();
}

function useAnswerAddNumber() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: submitSum,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["currentGame"] });
    },
  });
}

function useCurrentGame() {
  async function fetchCurrentGame(): Promise<AddNumbersGame | null> {
    const baseUrl = getApiBaseUrl();
    const response = await fetch(`${baseUrl}/api/add-numbers/current-game`);
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
    queryKey: ["currentGame"],
    queryFn: fetchCurrentGame,
    refetchInterval: 8 * 1000, // every 5 seconds
  });
}

export {
  useGameById,
  useSetupGame,
  useStartGame,
  useAnswerAddNumber,
  useCurrentGame,
};
