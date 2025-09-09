import { useQuery } from "@tanstack/react-query";
import { getApiBaseUrl } from "../config/backend";
import { OperatorKeys } from "../../../backend/bindings/OperatorKeys";

export default function usePubkey() {
  async function fetchOperatorKey(): Promise<OperatorKeys> {
    const baseUrl = getApiBaseUrl();
    const response = await fetch(`${baseUrl}/api/bitvmx/operator-keys`);
    if (!response.ok) {
      throw new Error("Failed to fetch game ID");
    }
    return await response.json();
  }

  return useQuery({
    queryKey: ["operatorKey"],
    queryFn: () => fetchOperatorKey(),
  });
}
