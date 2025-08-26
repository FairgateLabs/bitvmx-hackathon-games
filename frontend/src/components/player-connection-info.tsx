import { useState, useEffect } from "react";
import { Label } from "@/components/ui/label";
import { Button } from "@/components/ui/button";
import { Copy, RefreshCw } from "lucide-react";

interface PlayerConnectionInfoProps {
  networkSelected: string;
}

export function PlayerConnectionInfo({
  networkSelected,
}: PlayerConnectionInfoProps) {
  const [localIP, setLocalIP] = useState<string>("");
  const [localPort, setLocalPort] = useState<string>("");
  const [isLoading, setIsLoading] = useState(false);
  const [isExpanded, setIsExpanded] = useState(true);

  const getLocalIP = async () => {
    setIsLoading(true);
    try {
      //Try to get local IP from a service
      setLocalIP("127.0.0.1");
    } catch (error) {
      // Fallback to localhost if external service fails
      setLocalIP("127.0.0.1");
    }
    setIsLoading(false);
  };

  const getLocalPort = () => {
    // For development, we'll use a default port
    // In production, this could come from environment variables or be dynamic
    setLocalPort("3000");
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  useEffect(() => {
    getLocalIP();
    getLocalPort();
  }, []);

  return (
    <div className="p-4 bg-blue-50 border border-blue-200 rounded-lg">
      <h3
        className="font-semibold mb-3 text-blue-800 cursor-pointer"
        onClick={() => setIsExpanded(!isExpanded)}
      >
        🔌 Your Connection Information {isExpanded ? "▲" : "▼"}
      </h3>
      {isExpanded && (
        <>
          <p className="text-sm text-blue-700 mb-4">
            Share this information with other players so they can connect to
            your game.
          </p>

          <div className="space-y-3">
            <div className="flex items-center justify-between">
              <div>
                <Label className="text-blue-800">Your IP Address:</Label>
                <p className="font-mono text-sm bg-blue-100 p-2 rounded">
                  {localIP || "Loading..."}
                </p>
              </div>
              <div className="flex gap-2">
                <Button
                  size="sm"
                  variant="outline"
                  onClick={() => copyToClipboard(localIP)}
                  disabled={!localIP}
                >
                  <Copy className="h-4 w-4" />
                </Button>
                <Button
                  size="sm"
                  variant="outline"
                  onClick={getLocalIP}
                  disabled={isLoading}
                >
                  <RefreshCw
                    className={`h-4 w-4 ${isLoading ? "animate-spin" : ""}`}
                  />
                </Button>
              </div>
            </div>

            <div className="flex items-center justify-between">
              <div>
                <Label className="text-blue-800">Port:</Label>
                <p className="font-mono text-sm bg-blue-100 p-2 rounded">
                  {localPort || "3000"}
                </p>
              </div>
              <Button
                size="sm"
                variant="outline"
                onClick={() => copyToClipboard(localPort)}
                disabled={!localPort}
              >
                <Copy className="h-4 w-4" />
              </Button>
            </div>
          </div>

          <div className="mt-4 p-3 bg-blue-100 rounded">
            <p className="text-xs text-blue-800">
              💡 <strong>Tip:</strong> Other players need your IP address and
              port to join your game.
            </p>
          </div>
        </>
      )}
    </div>
  );
}
