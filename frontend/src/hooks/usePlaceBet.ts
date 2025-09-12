import { useMutation } from "@tanstack/react-query";
import { getApiBaseUrl } from "../config/backend";

interface PlaceBetRequest {
  program_id: string;
  amount: number; // in satoshis
}

const placeBet = async ({
  program_id,
  amount,
}: PlaceBetRequest): Promise<void> => {
  const baseUrl = getApiBaseUrl();
  const response = await fetch(`${baseUrl}/api/add-numbers/place-bet`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      program_id,
      amount,
    }),
  });

  if (!response.ok) {
    const errorData = await response.json().catch(() => ({}));
    console.error("Failed to place bet:", errorData);
  }
};

export const usePlaceBet = () => {
  return useMutation({
    mutationFn: placeBet,
  });
};
