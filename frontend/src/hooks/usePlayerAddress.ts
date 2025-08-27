import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";

// Types
interface PlayerAddress {
  address: string;
  balance: number;
  network: "regtest" | "testnet";
  isConnected: boolean;
  lastUpdated: string;
}

interface UpdateAddressParams {
  address: string;
  network: "regtest" | "testnet";
}

// Mock API functions (replace with real API calls)
const fetchPlayerAddress = async (): Promise<PlayerAddress> => {
  // Simulate API delay
  await new Promise((resolve) => setTimeout(resolve, 1000));

  // Mock data - replace with actual API call
  return {
    address: "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
    balance: 1.0,
    network: "regtest",
    isConnected: true,
    lastUpdated: new Date().toISOString(),
  };
};

const updatePlayerAddress = async (
  params: UpdateAddressParams
): Promise<PlayerAddress> => {
  // Simulate API delay
  await new Promise((resolve) => setTimeout(resolve, 800));

  // Mock update - replace with actual API call
  return {
    address: params.address,
    balance: 0.0, // Reset balance when changing address
    network: params.network,
    isConnected: true,
    lastUpdated: new Date().toISOString(),
  };
};

// Hook
export const usePlayerAddress = () => {
  const queryClient = useQueryClient();

  // Query for fetching player address
  const {
    data: playerAddress,
    isLoading,
    error,
    refetch,
  } = useQuery({
    queryKey: ["playerAddress"],
    queryFn: fetchPlayerAddress,
    staleTime: 5 * 60 * 1000, // 5 minutes
    gcTime: 10 * 60 * 1000, // 10 minutes (formerly cacheTime)
    retry: 3,
    retryDelay: (attemptIndex) => Math.min(1000 * 2 ** attemptIndex, 30000),
  });

  // Mutation for updating player address
  const updateAddressMutation = useMutation({
    mutationFn: updatePlayerAddress,
    onSuccess: (newAddress) => {
      // Update the cache with new data
      queryClient.setQueryData(["playerAddress"], newAddress);

      // Invalidate related queries
      queryClient.invalidateQueries({ queryKey: ["playerAddress"] });
    },
    onError: (error) => {
      console.error("Failed to update player address:", error);
    },
  });

  // Function to manually update address
  const updateAddress = (params: UpdateAddressParams) => {
    updateAddressMutation.mutate(params);
  };

  // Function to refresh address data
  const refreshAddress = () => {
    refetch();
  };

  // Function to check if address is valid
  const isValidAddress = (address: string): boolean => {
    // Basic validation - replace with proper Bitcoin address validation
    return address.length > 20 && address.startsWith("bc1");
  };

  return {
    // Data
    playerAddress,

    // Loading states
    isLoading,
    isUpdating: updateAddressMutation.isPending,

    // Error states
    error: error?.message,
    updateError: updateAddressMutation.error?.message,

    // Actions
    updateAddress,
    refreshAddress,
    isValidAddress,

    // Mutation state
    isSuccess: updateAddressMutation.isSuccess,
    isError: updateAddressMutation.isError,

    // Reset mutation state
    resetMutation: updateAddressMutation.reset,
  };
};
