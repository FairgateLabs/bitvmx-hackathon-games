import { useQuery } from "@tanstack/react-query";
import { getApiBaseUrl } from "../config/backend";
import { OperatorKeys } from "../../../backend/bindings/OperatorKeys";

const fetchAddressInfo = async (): Promise<OperatorKeys> => {
  const baseUrl = getApiBaseUrl();
  const response = await fetch(`${baseUrl}/api/bitvmx/operator-keys`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  });

  if (!response.ok) {
    throw new Error("Failed to fetch address info");
  }
  return await response.json();
};

// Hook for getting player address
export const usePubKey = () => {
  return useQuery({
    queryKey: ["pubKey"],
    queryFn: fetchAddressInfo,
  });
};
