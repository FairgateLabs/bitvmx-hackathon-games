import { useQuery } from "@tanstack/react-query";

interface Player2Answer {
  sum: number;
}

async function fetchPlayer2Answer(): Promise<Player2Answer> {
  // Mock API call - replace with actual endpoint
  // For now, simulate Player 2 responding after 5 seconds

  // const response = await fetch(`/api/player2/answer?gameId=${gameId}`);
  // if (!response.ok) {
  //   throw new Error("Failed to fetch Player 2's answer");
  // }
  // const data: Player2Answer = await response.json();
  // return data;
  await new Promise((resolve) => setTimeout(resolve, 5000));

  // Mock response - replace with actual API call
  return {
    sum: 8, // Mock sum (e.g., 5 + 3)
  };
}

export function usePlayer2Answer(gameId: string) {
  return useQuery({
    queryKey: ["player2-answer", gameId],
    queryFn: fetchPlayer2Answer,
    enabled: !!gameId,
    refetchInterval: false, // Don't poll, just simulate the response
  });
}
