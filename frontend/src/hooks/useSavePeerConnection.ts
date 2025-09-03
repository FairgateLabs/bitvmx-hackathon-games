import { useMutation } from "@tanstack/react-query";
import { getApiBaseUrl } from "../config/backend";

export const useSavePeerConnection = () => {
  // Send IP and Port to backend using react-query
  return useMutation({
    mutationFn: async ({
      networkAddress,
      peerId,
    }: {
      networkAddress: string;
      peerId: string;
    }) => {
      // const baseUrl = getApiBaseUrl();
      // const response = await fetch(`${baseUrl}/api/peer-connection`, {
      //   method: "POST",
      //   headers: {
      //     "Content-Type": "application/json",
      //   },
      //   body: JSON.stringify({ networkAddress, peerId }),
      // });
      // if (!response.ok) {
      //   throw new Error("Failed to save peer connection info");
      // }
      // return response.json();
    },
  });
};
