import { useMutation } from "@tanstack/react-query";

export const useSavePeerConnection = () => {
  // Send IP and Port to backend using react-query
  return useMutation({
    mutationFn: async ({ ip, port }: { ip: string; port: string }) => {
      const response = await fetch("/api/peer-connection", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ ip, port }),
      });
      if (!response.ok) {
        throw new Error("Failed to save peer connection info");
      }
      return response.json();
    },
  });
};
