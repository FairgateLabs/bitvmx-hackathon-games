import { useQuery } from "@tanstack/react-query";
import { getApiBaseUrl } from "../config/backend";
import { HealthResponse } from "../../../backend/bindings/HealthResponse";

/**
 * Hook to check the health status of the backend API
 * @param options - Optional query options
 * @returns Query result with health status and timestamp
 */
export function useHealth(options?: {
  enabled?: boolean;
  refetchInterval?: number;
  staleTime?: number;
}) {
  async function fetchHealth(): Promise<HealthResponse> {
    const baseUrl = getApiBaseUrl();
    const response = await fetch(`${baseUrl}/api/health`);

    if (!response.ok) {
      throw new Error(
        `Health check failed: ${response.status} ${response.statusText}`
      );
    }

    const data = await response.json();
    return data;
  }

  return useQuery({
    queryKey: ["health"],
    queryFn: fetchHealth,
    enabled: options?.enabled ?? true,
    refetchInterval: options?.refetchInterval,
    staleTime: options?.staleTime ?? 30000, // 30 seconds default
    retry: 3,
    retryDelay: (attemptIndex) => Math.min(1000 * 2 ** attemptIndex, 30000),
  });
}

/**
 * Hook to check if the backend is healthy (simplified boolean check)
 * @param options - Optional query options
 * @returns Boolean indicating if backend is healthy
 */
export function useIsHealthy(options?: {
  enabled?: boolean;
  refetchInterval?: number;
  staleTime?: number;
}) {
  const { data, isLoading, error } = useHealth(options);

  return {
    isHealthy: !isLoading && !error && data?.status === "healthy",
    isLoading,
    error,
    data,
  };
}
