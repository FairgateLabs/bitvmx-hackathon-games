import { useQuery } from "@tanstack/react-query";

export const usePeerConnection = () => {
  // Fetch IP and Port from backend using react-query
  return useQuery({
    queryKey: ["peerConnectionInfo"],
    queryFn: async () => {
      const response = await fetch("/api/peer-connection");
      if (!response.ok) {
        throw new Error("Failed to fetch peer connection info");
      }
      return response.json();
    },
  });
};
