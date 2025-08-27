import { useState } from "react";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Copy, RefreshCw } from "lucide-react";
import { useSavePeerConnection } from "@/hooks/useSavePeerConnection";

interface PeerConnectionInputProps {
  networkSelected: string;
}

export function PeerConnectionInput({
  networkSelected,
}: PeerConnectionInputProps) {
  const [peerIP, setPeerIP] = useState("");
  const [peerPort, setPeerPort] = useState("");
  const [isExpanded, setIsExpanded] = useState(true);
  const [inputsDisabled, setInputsDisabled] = useState(false);
  const [successMessage, setSuccessMessage] = useState("");
  const { mutate: savePeerConnection } = useSavePeerConnection();

  const isValidIP = (ip: string) => {
    const ipRegex =
      /^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/;
    return ipRegex.test(ip);
  };

  const isValidPort = (port: string) => {
    const portNumber = parseInt(port, 10);
    return portNumber > 0 && portNumber <= 65535;
  };

  const handleSetConnection = () => {
    savePeerConnection({ ip: peerIP, port: peerPort });
    setInputsDisabled(true);
    setSuccessMessage("Connection successfully established!");
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
                disabled={inputsDisabled}
              />
              {!isValidIP(peerIP) && peerIP && (
                <p className="text-red-600 text-sm">
                  Invalid IP address format.
                </p>
              )}
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
                disabled={inputsDisabled}
              />
              {!isValidPort(peerPort) && peerPort && (
                <p className="text-red-600 text-sm">Invalid port number.</p>
              )}
            </div>

            <Button
              onClick={handleSetConnection}
              disabled={
                !isValidIP(peerIP) || !isValidPort(peerPort) || inputsDisabled
              }
              className="w-full bg-green-600 hover:bg-green-700"
            >
              🔗 Set Connection
            </Button>

            {successMessage && (
              <p className="text-green-600 text-sm mt-2">{successMessage}</p>
            )}
          </div>
        </>
      )}
    </div>
  );
}
