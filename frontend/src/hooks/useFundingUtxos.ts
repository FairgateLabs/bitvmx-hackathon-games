import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { getApiBaseUrl } from "../config/backend";
import { Utxo } from "../../../backend/bindings/Utxo";
import { FundingUtxosResponse } from "../../../backend/bindings/FundingUtxosResponse";

// Function to fetch my funding UTXO
const fetchFundingUtxos = async (
  uuid: string
): Promise<FundingUtxosResponse> => {
  const baseUrl = getApiBaseUrl();
  const response = await fetch(
    `${baseUrl}/api/add-numbers/fundings_utxos/${uuid}`,
    {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
      },
    }
  );

  if (!response.ok) {
    throw new Error("Failed to fetch my funding UTXO");
  }

  const data: FundingUtxosResponse = await response.json();
  console.log(data);
  return data; // Extract utxo from FundingUtxosResponse
};

// Hook for getting my funding UTXO
export const useFundingUtxos = (uuid: string) => {
  return useQuery({
    queryKey: ["fundingUtxos", uuid],
    queryFn: () => fetchFundingUtxos(uuid),
    enabled: !!uuid, // Only run query if uuid is provided
  });
};

// Function to send other participant's UTXO
const saveFundingUtxos = async ({
  uuid,
  otherUtxo,
}: {
  uuid: string;

  otherUtxo: Utxo;
}): Promise<void> => {
  const baseUrl = getApiBaseUrl();
  const response = await fetch(`${baseUrl}/api/add-numbers/fundings_utxos`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ otherUtxo }),
  });

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
      queryClient.invalidateQueries({ queryKey: ["utxoExchange"] });
    },
  });
};
