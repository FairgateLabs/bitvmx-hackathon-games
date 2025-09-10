import { useMutation, useQuery } from "@tanstack/react-query";
import { getApiBaseUrl } from "../config/backend";
import { P2PAddress } from "../../../backend/bindings/P2PAddress";
import { SetupParticipantsRequest } from "../../../backend/bindings/SetupParticipantsRequest";

const useSaveParticipantInfo = () => {
  return useMutation({
    mutationFn: async ({
      p2p_addresses: participants_addresses,
      operator_keys,
      aggregated_id,
    }: {
      p2p_addresses: P2PAddress[];
      operator_keys: string[];
      aggregated_id: string;
    }) => {
      let data: SetupParticipantsRequest = {
        aggregated_id,
        participants_addresses: participants_addresses,
        participants_keys: operator_keys,
        leader_idx: 0,
      };

      const baseUrl = getApiBaseUrl();
      const response = await fetch(
        `${baseUrl}/api/add-numbers/setup-participants`,
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify(data),
        }
      );

      if (!response.ok) {
        throw new Error("Failed to save peer connection info");
      }
    },
  });
};

const useGetParticipantInfo = (uuid: string | null) => {
  return useQuery({
    queryKey: ["participantInfo", uuid],
    queryFn: async () => {
      if (!uuid) {
        throw new Error("UUID is required to fetch participant info");
      }
      console.log("Fetching participant info for UUID:", uuid);
      const baseUrl = getApiBaseUrl();
      const response = await fetch(
        `${baseUrl}/api/bitvmx/aggregated-key/${uuid}`,
        {
          method: "GET",
          headers: {
            "Content-Type": "application/json",
          },
        }
      );

      if (!response.ok) {
        throw new Error("Failed to fetch participant info");
      }

      return response.json();
    },
    enabled: !!uuid,
  });
};

export { useSaveParticipantInfo, useGetParticipantInfo };
