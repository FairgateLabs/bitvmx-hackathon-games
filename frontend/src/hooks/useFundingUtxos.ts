import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { getApiBaseUrl } from "../config/backend";
import { FundingUtxoRequest } from "../../../backend/bindings/FundingUtxoRequest";

// Function to send other participant's UTXO
const saveFundingUtxos = async (data: FundingUtxoRequest): Promise<void> => {
  const baseUrl = getApiBaseUrl();
  const response = await fetch(
    `${baseUrl}/api/add-numbers/setup-funding-utxo`,
    {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    }
  );

  if (!response.ok) {
    throw new Error("Failed to send other participant's UTXO");
  }

  return response.json();
};

// Hook for sending other participant's UTXO
export const useSaveFundingUtxos = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: saveFundingUtxos,
    onSuccess: () => {
      // Invalidate any relevant queries
      queryClient.invalidateQueries({ queryKey: ["fundingExchange"] });
    },
  });
};
