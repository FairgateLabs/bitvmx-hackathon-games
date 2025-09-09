import { useMutation } from "@tanstack/react-query";
import { getApiBaseUrl } from "../config/backend";
import { AggregatedKeySubmission } from "../../../backend/bindings/AggregatedKeySubmission";
import { P2PAddress } from "../../../backend/bindings/P2PAddress";

export const useSavePeerConnection = () => {
  // Send IP and Port to backend using react-query
  return useMutation({
    mutationFn: async ({
      p2p_addresses,
      operator_keys,
    }: {
      p2p_addresses: P2PAddress[];
      operator_keys: string[];
    }) => {
      let uuid = crypto.randomUUID();
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
