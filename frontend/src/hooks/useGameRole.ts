import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { PlayerRole } from "@/types/game";

// Hook for getting the current role
const useGameRole = () => {
  async function fetchGameRole() {
    const queryClient = useQueryClient();
    const current = queryClient.getQueryData<PlayerRole>(["role"]);
    return current || PlayerRole.Player1;
  }

  return useQuery<PlayerRole | null>({
    queryKey: ["role"],
    queryFn: fetchGameRole,
    initialData: PlayerRole.Player1,
  });
};

// Hook for saving the role using mutation
const useSaveGameRole = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (role: PlayerRole) => {
      queryClient.setQueryData(["role"], role);
      console.log("saved role", role);
    },
  });
};

export { useGameRole, useSaveGameRole };
