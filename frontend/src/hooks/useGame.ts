import { useQuery } from "@tanstack/react-query";

export function useGame() {
  async function fetchGameId() {
    // Simulate fetching game ID from a request
    const response = await fetch("/api/game-id");
    if (!response.ok) {
      throw new Error("Failed to fetch game ID");
    }
    const data = await response.json();
    return data.gameId;
  }

  return useQuery({
    queryKey: ["gameId"],
    queryFn: fetchGameId,
  });
}
