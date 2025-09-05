import { useMutation } from "@tanstack/react-query";
import { getApiBaseUrl } from "../config/backend";
import { AggregatedKeySubmission } from "../../../backend/bindings/AggregatedKeySubmission";

export const useSavePeerConnection = () => {
  // Send IP and Port to backend using react-query
  return useMutation({
    mutationFn: async ({
      address,
      peer_id,
      operator_keys,
    }: {
      address: string;
      peer_id: string;
      operator_keys: string[];
    }) => {
      let uuid = crypto.randomUUID();
      let data: AggregatedKeySubmission = {
        uuid,
        p2p_addresses: [{ address, peer_id }],
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
