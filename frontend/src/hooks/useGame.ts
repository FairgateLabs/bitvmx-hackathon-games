import { useQuery } from "@tanstack/react-query";
import { getApiBaseUrl } from "../config/backend";

export function useGame() {
  async function fetchGameId() {
    // const baseUrl = getApiBaseUrl();
    // const response = await fetch(`${baseUrl}/api/game-id`);
    // if (!response.ok) {
    //   throw new Error("Failed to fetch game ID");
    // }
    // const data = await response.json();
    // return data.gameId;

    return "123";
  }

  return useQuery({
    queryKey: ["gameId"],
    queryFn: fetchGameId,
  });
}
