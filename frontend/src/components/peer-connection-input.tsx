import { useState } from "react";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Copy, RefreshCw } from "lucide-react";

interface PeerConnectionInputProps {
  networkSelected: string;
  onConnectionSet: (ip: string, port: string) => void;
}

export function PeerConnectionInput({
  networkSelected,
  onConnectionSet,
}: PeerConnectionInputProps) {
  const [peerIP, setPeerIP] = useState("");
  const [peerPort, setPeerPort] = useState("");
  const [isExpanded, setIsExpanded] = useState(true);

  const handleSetConnection = () => {
    if (peerIP && peerPort) {
      onConnectionSet(peerIP, peerPort);
    }
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  return (
    <div className="p-4 bg-green-50 border border-green-200 rounded-lg">
      <h3
        className="font-semibold mb-3 text-green-800 cursor-pointer"
        onClick={() => setIsExpanded(!isExpanded)}
      >
        🔗 Other Player's Connection {isExpanded ? "▲" : "▼"}
      </h3>
      {isExpanded && (
        <>
          <p className="text-sm text-green-700 mb-4">
            Enter the IP address and port of the other player to connect to
            their game and allow bitvmx client to connect to it.
          </p>

          <div className="space-y-3">
            <div>
              <Label htmlFor="peerIP" className="text-green-800">
                Other Player's IP Address:
              </Label>
              <Input
                id="peerIP"
                value={peerIP}
                onChange={(e) => setPeerIP(e.target.value)}
                placeholder="e.g., 192.168.1.100"
                className="mt-1"
              />
            </div>

            <div>
              <Label htmlFor="peerPort" className="text-green-800">
                Other Player's Port:
              </Label>
              <Input
                id="peerPort"
                value={peerPort}
                onChange={(e) => setPeerPort(e.target.value)}
                placeholder="e.g., 3000"
                className="mt-1"
              />
            </div>

            <Button
              onClick={handleSetConnection}
              disabled={!peerIP || !peerPort}
              className="w-full bg-green-600 hover:bg-green-700"
            >
              🔗 Set Connection
            </Button>
          </div>
        </>
      )}
    </div>
  );
}
