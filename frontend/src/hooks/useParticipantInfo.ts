import { useMutation, useQuery } from "@tanstack/react-query";
import { getApiBaseUrl } from "../config/backend";
import { AggregatedKeySubmission } from "../../../backend/bindings/AggregatedKeySubmission";
import { P2PAddress } from "../../../backend/bindings/P2PAddress";
import { UUID } from "crypto";

const useSaveParticipantInfo = () => {
  return useMutation({
    mutationFn: async ({
      p2p_addresses,
      operator_keys,
      uuid,
    }: {
      p2p_addresses: P2PAddress[];
      operator_keys: string[];
      uuid: string;
    }) => {
      let data: AggregatedKeySubmission = {
        uuid,
        p2p_addresses: p2p_addresses,
        operator_keys: operator_keys,
        leader_idx: 0,
      };

      const baseUrl = getApiBaseUrl();
      const response = await fetch(`${baseUrl}/api/bitvmx/aggregated-key`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
      });

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
