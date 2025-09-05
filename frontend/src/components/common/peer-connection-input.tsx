import { useState } from "react";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { useSavePeerConnection } from "@/hooks/useSavePeerConnection";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { useNextGameState } from "@/hooks/useGameState";
import { GameState } from "@/types/game";

export function PeerConnectionInput() {
  const [networkAddress, setPeerIP] = useState("");
  const [peerId, setPeerId] = useState("");
  const [pubKey, setPubKey] = useState("");
  const [isOpen, setIsOpen] = useState(true);
  const [inputsDisabled, setInputsDisabled] = useState(false);
  const [successMessage, setSuccessMessage] = useState("");
  const { mutate: savePeerConnection } = useSavePeerConnection();
  const { mutate: nextState } = useNextGameState();

  const isValidNetworkAddress = (networkAddress: string) => {
    const [ip, port] = networkAddress.split(":");
    const ipRegex =
      /^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/;
    return ipRegex.test(ip) && isValidPort(port);
  };

  const isValidPort = (port: string) => {
    const portNumber = parseInt(port, 10);
    return portNumber > 0 && portNumber <= 65535;
  };

  const isValidPeeId = (key: string): boolean => {
    const hexRegex = /^[0-9a-fA-F]+$/; // only hex chars
    return hexRegex.test(key) && key.length % 2 === 0;
  };

  const isValidPubKey = (key: string): boolean => {
    const hexRegex = /^[0-9a-fA-F]+$/; // only hex chars
    return hexRegex.test(key) && key.length === 66; // typical length for uncompressed public key
  };

  const handleSetConnection = () => {
    savePeerConnection({
      address: networkAddress,
      peer_id: peerId,
      operator_keys: [pubKey],
    });
    setInputsDisabled(true);
    setSuccessMessage("Connection successfully established!");
    nextState(GameState.SetupProgram);
  };

  return (
    <div className="p-4 bg-white border border-gray-200 rounded-lg">
      <Collapsible open={isOpen} onOpenChange={setIsOpen}>
        <CollapsibleTrigger asChild>
          <h3 className="font-semibold mb-3 text-gray-800 cursor-pointer hover:text-gray-900">
            🔗 Other Player's Connection
          </h3>
        </CollapsibleTrigger>
        <CollapsibleContent className="flex flex-col gap-3">
          <p className="text-sm text-gray-700 mb-4">
            Enter the IP address and port of the other player to connect to
            their game and allow bitvmx client to connect to it.
          </p>

          <div>
            <Label htmlFor="pubKey" className="text-gray-800 ">
              Public Key:
            </Label>
            <Input
              id="pubKey"
              value={pubKey}
              onChange={(e) => setPubKey(e.target.value)}
              placeholder="e.g., 04bfcab2c3a3f..."
              className="mt-1"
              disabled={inputsDisabled}
            />
            {!isValidPubKey(pubKey) && pubKey && (
              <p className="text-red-600 text-sm">Invalid public key format.</p>
            )}
          </div>

          <div>
            <Label htmlFor="peerIP" className="text-gray-800">
              Network Address:
            </Label>
            <Input
              id="peerIP"
              value={networkAddress}
              onChange={(e) => setPeerIP(e.target.value)}
              placeholder="e.g., 192.168.1.100"
              className="mt-1"
              disabled={inputsDisabled}
            />
            {!isValidNetworkAddress(networkAddress) && networkAddress && (
              <p className="text-red-600 text-sm">
                Invalid Network Address format (e.g., 192.168.1.100:3000).
              </p>
            )}
          </div>

          <div>
            <Label htmlFor="peerId" className="text-gray-800">
              Peer ID:
            </Label>
            <Input
              id="peerId"
              value={peerId}
              onChange={(e) => setPeerId(e.target.value)}
              placeholder="e.g., 30820122300d06092a864886f70d010101050003820b..."
              className="mt-1"
              disabled={inputsDisabled}
            />
            {!isValidPeeId(peerId) && peerId && (
              <p className="text-red-600 text-sm">Invalid peer ID format.</p>
            )}
          </div>

          <Button
            onClick={handleSetConnection}
            disabled={!isValidNetworkAddress(networkAddress) || inputsDisabled}
            className="w-full bg-gray-600 hover:bg-gray-700"
          >
            🔗 Set Connection
          </Button>

          {successMessage && (
            <div className="p-4 bg-green-50 border border-green-200 rounded-lg">
              <h3 className="font-semibold mb-2 text-green-800">
                ✅ Connection Successful
              </h3>
              <p className="text-sm text-green-700">{successMessage}</p>
            </div>
          )}

          {!successMessage && (
            <div className="p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
              <h3 className="font-semibold mb-2 text-yellow-800">
                ⚠️ Connection Setup Required
              </h3>
              <p className="text-sm text-yellow-700">
                Please complete the connection setup by entering the other
                player's IP address and port above.
              </p>
            </div>
          )}
        </CollapsibleContent>
      </Collapsible>
    </div>
  );
}
