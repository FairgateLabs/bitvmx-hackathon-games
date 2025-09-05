import { useQuery } from "@tanstack/react-query";
import { getApiBaseUrl } from "../config/backend";
import { P2PAddress } from "../../../backend/bindings/P2PAddress";

const fetchCommunicationInfo = async (): Promise<P2PAddress> => {
  const baseUrl = getApiBaseUrl();
  const response = await fetch(`${baseUrl}/api/bitvmx/comm-info`);
  if (!response.ok) {
    throw new Error("Failed to fetch peer connection info");
  }
  return await response.json();
};

export const useCommunicationInfo = () => {
  // Fetch IP and Port from backend using react-query
  return useQuery({
    queryKey: ["peerConnectionInfo"],
    queryFn: fetchCommunicationInfo,
  });
};
