import { useQuery } from "@tanstack/react-query";
import { getApiBaseUrl } from "../config/backend";
import { ProtocolVisualizationResponse } from "../../../backend/bindings/ProtocolVisualizationResponse";
import { instance } from "@viz-js/viz";

const fetchProtocolVisualization = async (
  program_id: string
): Promise<string> => {
  const baseUrl = getApiBaseUrl();
  const response = await fetch(
    `${baseUrl}/api/bitvmx/protocol/visualization/${program_id}`,
    {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
      },
    }
  );

  if (!response.ok) {
    throw new Error("Failed to fetch protocol visualization");
  }

  let data: ProtocolVisualizationResponse = await response.json();
  return data.visualization;
};

// Hook for getting protocol visualization
export const useProtocolVisualization = (program_id: string | undefined) => {
  return useQuery({
    queryKey: ["protocolVisualization", program_id],
    queryFn: () => fetchProtocolVisualization(program_id ?? ""),
    enabled: !!program_id,
  });
};
