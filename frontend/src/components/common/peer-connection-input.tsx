import { useState } from "react";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { useSaveParticipantInfo } from "@/hooks/useParticipantInfo";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { PlayerRole } from "@/types/game";
import { useCommunicationInfo } from "@/hooks/useCommunicationInfo";
import usePubkey from "@/hooks/usePubkey";
import { useGameRole } from "@/hooks/useGameRole";

export function PeerConnectionInput({ gameId }: { gameId: string | null }) {
  const [address, setAddress] = useState("");
  const [peerId, setPeerId] = useState("");
  const [pubKey, setPubKey] = useState("");
  const [gameUUID, setGameUUID] = useState<string | null>(gameId);
  const { data: role } = useGameRole();
  const [isOpen, setIsOpen] = useState(true);
  const [inputsDisabled, setInputsDisabled] = useState(false);
  const [successMessage, setSuccessMessage] = useState("");
  const { mutate: savePeerConnection } = useSaveParticipantInfo();
  const { data: peerConnectionInfo } = useCommunicationInfo();
  const { data: operatorKey } = usePubkey();

  const isValidNetworkAddress = (networkAddress: string) => {
    const regex =
      /^\/ip4\/(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\/tcp\/([0-9]{1,5})$/;
    const match = networkAddress.match(regex);
    if (!match) return false;
    const port = parseInt(match[5], 10);
    return port > 0 && port <= 65535;
  };

  const isValidPeeId = (key: string): boolean => {
    const hexRegex = /^[0-9a-fA-F]+$/; // only hex chars
    return hexRegex.test(key) && key.length % 2 === 0;
  };

  const isValidPubKey = (key: string): boolean => {
    const hexRegex = /^[0-9a-fA-F]+$/; // only hex chars
    return hexRegex.test(key) && key.length === 66;
  };

  const isUUIDValid = (uuid: string) => {
    const uuidRegex =
      /^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;
    return uuidRegex.test(uuid);
  };

  const handleSetConnection = () => {
    if (!gameUUID) {
      return;
    }

    savePeerConnection({
      p2p_addresses: [
        {
          address: peerConnectionInfo?.address ?? "",
          peer_id: peerConnectionInfo?.peer_id ?? "",
        },
        { address, peer_id: peerId },
      ],
      operator_keys: [operatorKey?.pub_key ?? "", pubKey],
      uuid: gameUUID,
    });
    setInputsDisabled(true);
    setSuccessMessage("Connection successfully established!");
    // nextState(GameState.SetupProgram);
  };

  return (
    <div className="p-4 bg-white border border-gray-200 rounded-lg">
      <Collapsible open={isOpen} onOpenChange={setIsOpen}>
        <CollapsibleTrigger asChild>
          <h3 className="font-semibold mb-3 text-gray-800 cursor-pointer hover:text-gray-900">
            🔗 Other Player's Setup Data
          </h3>
        </CollapsibleTrigger>
        <CollapsibleContent className="flex flex-col gap-3">
          <p className="text-sm text-gray-700">
            {role === PlayerRole.Player1
              ? "Enter the Public Key, Network Address and Peer ID of the other player to connect to your game."
              : "Enter the Game UUID, Public Key, Network Address and Peer ID of the other player to join their game."}
          </p>

          <p className="text-sm text-gray-700 mb-4">
            BitVMX will use all this information to connect to the other client
            and start computing the program.
          </p>

          {role === PlayerRole.Player1 ? (
            <div>
              <Label htmlFor="gameUUID" className="text-gray-800">
                Game UUID:
              </Label>
              <p className="font-mono text-sm bg-gray-100 p-2 rounded overflow-hidden text-ellipsis whitespace-nowrap max-w-[500px]">
                {gameId ?? "No game active"}
              </p>
            </div>
          ) : (
            <div>
              <Label htmlFor="gameUUID" className="text-gray-800">
                Game UUID:
              </Label>
              <Input
                id="gameUUID"
                value={gameUUID ?? ""}
                onChange={(e) => setGameUUID(e.target.value)}
                placeholder="e.g., 123e4567-e89b-12d3-a456-426614174000"
                className="mt-1"
                disabled={inputsDisabled}
              />
              {gameUUID && !isUUIDValid(gameUUID) && (
                <p className="text-red-600 text-sm">Invalid UUID format.</p>
              )}
            </div>
          )}

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
              value={address}
              onChange={(e) => setAddress(e.target.value)}
              placeholder="e.g., /ip4/127.0.0.1/tcp/61181"
              className="mt-1"
              disabled={inputsDisabled}
            />
            {!isValidNetworkAddress(address) && address && (
              <p className="text-red-600 text-sm">
                Invalid Network Address format (e.g., /ip4/127.0.0.1/tcp/61181).
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
            disabled={
              !isValidNetworkAddress(address) ||
              !isValidPubKey(pubKey) ||
              !isValidPeeId(peerId) ||
              !isUUIDValid(gameUUID ?? "") ||
              inputsDisabled
            }
            className="w-full bg-gray-600 hover:bg-gray-700"
          >
            🔗 Setup Data
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
                Ensure you enter the other player's Network Address, Peer ID and
                Public Key to finalize the connection setup.
              </p>
            </div>
          )}
        </CollapsibleContent>
      </Collapsible>
    </div>
  );
}
