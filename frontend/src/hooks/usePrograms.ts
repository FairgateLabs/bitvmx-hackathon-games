import { useMutation } from "@tanstack/react-query";
import { getApiBaseUrl } from "../config/backend";

// API function to fetch a specific program by ID
const saveProgramById = async (programId: string): Promise<any> => {
  // const baseUrl = getApiBaseUrl();
  // await fetch(`${baseUrl}/api/programs`, {
  //   method: "POST",
  //   headers: {
  //     "Content-Type": "application/json",
  //   },
  //   body: JSON.stringify({ programId }),
  // });
  // if (!response.ok) {
  //   throw new Error(`Failed to fetch program: ${response.status}`);
  // }
};

// Hook for fetching a specific program by ID using mutation
export const useProgramMutation = () => {
  return useMutation({
    mutationFn: (programId: string) => saveProgramById(programId),
  });
};
