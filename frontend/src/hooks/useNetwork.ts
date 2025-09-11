import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { NetworkType } from "@/types/network";

export function useNetworkQuery() {
  const queryClient = useQueryClient();

  const getNetwork = async (): Promise<NetworkType | null> => {
    const cachedNetwork = queryClient.getQueryData<NetworkType>(["network"]);
    return cachedNetwork ?? null;
  };

  return useQuery<NetworkType | null>({
    queryKey: ["network"],
    queryFn: getNetwork,
  });
}

export function useNetworkMutation() {
  const queryClient = useQueryClient();

  return useMutation<NetworkType, unknown, NetworkType>({
    mutationFn: async (newNetwork) => {
      // Simulate saving the network locally
      return newNetwork;
    },
    onSuccess: (newNetwork) => {
      queryClient.setQueryData(["network"], newNetwork);
    },
  });
}

const useNetwork = {
  useNetworkQuery,
  useNetworkMutation,
};

export default useNetwork;
