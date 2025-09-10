import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { getApiBaseUrl } from "../config/backend";
import { Utxo } from "../../../backend/bindings/Utxo";
import { SendFundsRequest } from "../../../backend/bindings/SendFundsRequest";

// Function to fetch my funding UTXO
const fetchMyFundingUtxo = async (uuid: string): Promise<Utxo> => {
  const baseUrl = getApiBaseUrl();
  const response = await fetch(
    `${baseUrl}/api/bitvmx/my-participant_funding_utxo/${uuid}`,
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

  const data = await response.json();
  console.log(data);
  return data.utxo; // Extract utxo from MyFundingUtxoResponse
};

// Hook for getting my funding UTXO
export const useMyFundingUtxo = (uuid: string) => {
  return useQuery({
    queryKey: ["myFundingUtxo", uuid],
    queryFn: () => fetchMyFundingUtxo(uuid),
    enabled: !!uuid, // Only run query if uuid is provided
  });
};

// Function to send other participant's UTXO
const sendOtherParticipantUtxo = async ({
  uuid,
  otherUtxo,
}: {
  uuid: string;
  otherUtxo: Utxo;
}): Promise<void> => {
  const baseUrl = getApiBaseUrl();
  const response = await fetch(
    `${baseUrl}/api/bitvmx/other_participant_funding_utxo/${uuid}`,
    {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(otherUtxo),
    }
  );

  if (!response.ok) {
    throw new Error("Failed to send other participant's UTXO");
  }

  return response.json();
};

// Hook for sending other participant's UTXO
export const useSendOtherUtxo = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: sendOtherParticipantUtxo,
    onSuccess: () => {
      // Invalidate any relevant queries
      queryClient.invalidateQueries({ queryKey: ["utxoExchange"] });
    },
  });
};
