import { useQuery } from "@tanstack/react-query";
import { getApiBaseUrl } from "../config/backend";

const fetchProtocolVisualization = async (
  program_id: string
): Promise<string> => {
  const baseUrl = getApiBaseUrl();
  console.log("program_id", program_id);
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
    console.log("response", response);
    throw new Error("Failed to fetch protocol visualization");
  }
  const data = await response.json();

  console.log("dataaaaaa", data);
  return data.visualization;
};

// Hook for getting protocol visualization
export const useProtocolVisualization = (program_id: string | undefined) => {
  return useQuery({
    queryKey: ["protocolVisualization", program_id],
    queryFn: () => fetchProtocolVisualization(program_id ?? ""),
    enabled: !!program_id,
    retry: true,
  });
};
