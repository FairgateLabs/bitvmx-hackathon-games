import { useMutation, useQueryClient } from "@tanstack/react-query";

// Types
interface SaveProgramParams {
  programId: string;
  numbers: {
    number1: number;
    number2: number;
  };
  network: "regtest" | "testnet";
  playerAddress: string;
}

interface SaveProgramResponse {
  success: boolean;
  programId: string;
  message: string;
  timestamp: string;
}

// API function to save program
const saveProgramToBackend = async (
  params: SaveProgramParams,
): Promise<SaveProgramResponse> => {
  const response = await fetch("/api/programs", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(params),
  });

  if (!response.ok) {
    const errorData = await response.json().catch(() => ({}));
    throw new Error(
      errorData.message || `Failed to save program: ${response.status}`,
    );
  }

  return response.json();
};

// Hook for saving program
export const useSaveProgram = () => {
  const queryClient = useQueryClient();

  const mutation = useMutation({
    mutationFn: saveProgramToBackend,
    onSuccess: (data, variables) => {
      // Update cache with new program data
      queryClient.setQueryData(["programs", variables.programId], data);

      // Invalidate related queries
      queryClient.invalidateQueries({ queryKey: ["programs"] });
      queryClient.invalidateQueries({ queryKey: ["playerPrograms"] });

      console.log("Program saved successfully:", data);
    },
    onError: (error) => {
      console.error("Failed to save program:", error);
    },
  });

  const saveProgram = (params: SaveProgramParams) => {
    mutation.mutate(params);
  };

  const saveProgramAsync = (params: SaveProgramParams) => {
    return mutation.mutateAsync(params);
  };

  return {
    // Mutation state
    isPending: mutation.isPending,
    isSuccess: mutation.isSuccess,
    isError: mutation.isError,
    isIdle: mutation.isIdle,

    // Data
    data: mutation.data,

    // Error
    error: mutation.error?.message,

    // Actions
    saveProgram,
    saveProgramAsync,

    // Reset
    reset: mutation.reset,

    // Variables (last mutation parameters)
    variables: mutation.variables,
  };
};
