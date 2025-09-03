/**
 * Backend configuration based on frontend port
 * Port 3000 -> Backend 1 (default)
 * Port 3001 -> Backend 2
 */

export interface BackendConfig {
  baseUrl: string;
  name: string;
  port: number;
}

// Backend configurations
const BACKEND_CONFIGS: Record<string, BackendConfig> = {
  "3000": {
    baseUrl: "http://localhost:8080",
    name: "Backend 1",
    port: 8080,
  },
  "3001": {
    baseUrl: "http://localhost:8081",
    name: "Backend 2",
    port: 8081,
  },
};

/**
 * Get the current frontend port
 */
function getCurrentPort(): string {
  if (typeof window === "undefined") {
    // Server-side rendering
    return "3000"; // Default port
  }

  return window.location.port || "3000";
}

/**
 * Get backend configuration for current frontend instance
 */
export function getBackendConfig(): BackendConfig {
  const currentPort = getCurrentPort();
  const config = BACKEND_CONFIGS[currentPort];

  if (!config) {
    console.warn(
      `No backend config found for port ${currentPort}, using default`
    );
    return BACKEND_CONFIGS["3000"];
  }

  return config;
}

/**
 * Get the base URL for API calls
 */
export function getApiBaseUrl(): string {
  return getBackendConfig().baseUrl;
}

/**
 * Get the backend name for display purposes
 */
export function getBackendName(): string {
  return getBackendConfig().name;
}

/**
 * Get the backend port
 */
export function getBackendPort(): number {
  return getBackendConfig().port;
}

/**
 * Check if we're running on a specific frontend port
 */
export function isRunningOnPort(port: string | number): boolean {
  return getCurrentPort() === port.toString();
}

/**
 * Get all available backend configurations
 */
export function getAllBackendConfigs(): BackendConfig[] {
  return Object.values(BACKEND_CONFIGS);
}
