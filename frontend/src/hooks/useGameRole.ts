import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { GameRole, Role } from "@/components/common/game-role-selector";

// Hook for getting the current role
const useGameRole = () => {
  return useQuery<Role>({
    queryKey: ["role"],
    queryFn: async () => {
      const queryClient = useQueryClient();
      const current = queryClient.getQueryData<Role>(["role"]);
      console.log("current ROLE", current);
      return current || null;
    },
  });
};

// Hook for saving the role using mutation
const useSaveGameRole = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (role: GameRole) => {
      queryClient.setQueryData(["role"], role);
      console.log("saved role", role);
    },
  });
};

export { useGameRole, useSaveGameRole };
