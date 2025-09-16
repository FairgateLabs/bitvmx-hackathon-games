import { useIsHealthy } from "@/hooks/useHealth";

interface BackendStatusProps {
  children: React.ReactNode;
  className?: string;
}

/**
 * Component that wraps children and shows "Connecting to backend..."
 * when the backend is not working
 */
export function BackendStatus({
  children,
  className = "container mx-auto p-6 max-w-4xl",
}: BackendStatusProps) {
  const {
    isHealthy,
    isLoading: isHealthLoading,
    error: healthError,
  } = useIsHealthy();

  // Check if backend is loading
  if (isHealthLoading) {
    return (
      <div className={className}>
        <div className="text-center">
          <p className="text-lg">🔄 Connecting to backend...</p>
        </div>
      </div>
    );
  }

  // Check if there is a health error
  if (healthError) {
    return (
      <div className={className}>
        <div className="text-center">
          <p className="text-lg">
            ❌ Error connecting to backend. <br />
            Please check is the backend running and try again.
          </p>
        </div>
      </div>
    );
  }

  // Check if backend is not healthy
  if (!isHealthy) {
    return (
      <div className={className}>
        <div className="text-center">
          <p className="text-lg">⚠️ Backend is not healthy.</p>
        </div>
      </div>
    );
  }

  return <>{children}</>;
}
