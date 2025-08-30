import { useQuery } from "@tanstack/react-query";
import { NetworkType } from "@/types/network";

export function useNetwork() {
  return useQuery<NetworkType>({
    queryKey: ["network"],
    queryFn: async () => {
      // Hardcoded network type for now
      return NetworkType.Regtest;
    },
    initialData: NetworkType.Regtest,
  });
}
