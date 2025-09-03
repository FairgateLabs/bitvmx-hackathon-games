import { useQuery } from "@tanstack/react-query";
import { getApiBaseUrl } from "../config/backend";

const fetchPeerConnectionInfo = async () => {
  // const baseUrl = getApiBaseUrl();
  // const response = await fetch(`${baseUrl}/api/peer-connection`);
  // if (!response.ok) {
  //   throw new Error("Failed to fetch peer connection info");
  // }
  // return response.json();
  return { networkAddress: "127.0.0.1:8080", peerId: "123" };
};

export const usePeerConnection = () => {
  // Fetch IP and Port from backend using react-query
  return useQuery({
    queryKey: ["peerConnectionInfo"],
    queryFn: fetchPeerConnectionInfo,
  });
};
