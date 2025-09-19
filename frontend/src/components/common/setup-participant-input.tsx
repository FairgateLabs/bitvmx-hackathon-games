import { useState } from "react";
import { Button } from "@/components/ui/button";
import { useSaveParticipantInfo } from "@/hooks/useParticipantInfo";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { EnumPlayerRole } from "@/types/game";
import { useCommunicationInfo } from "@/hooks/useCommunicationInfo";
import usePubkey from "@/hooks/usePubkey";
import { Textarea } from "@/components/ui/textarea";
import { PlayerRole } from "../../../../backend/bindings/PlayerRole";

interface PeerConnectionData {
  publicKey: string;
  networkAddress: string;
  peerId: string;
  aggregatedId?: string; // Make aggregatedId optional
}

export function SetupParticipantInput({
  aggregatedId,
  role,
}: {
  role: PlayerRole;
  aggregatedId: string;
}) {
  const [jsonInput, setJsonInput] = useState("");
  const [parsedData, setParsedData] = useState<PeerConnectionData | null>(null);
  const [jsonError, setJsonError] = useState("");
  const [isOpen, setIsOpen] = useState(true);
  const [inputsDisabled, setInputsDisabled] = useState(false);
  const [successMessage, setSuccessMessage] = useState(false);
  const { mutate: savePeerConnection, isPending: isSavingParticipantInfo } =
    useSaveParticipantInfo();
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

  const isValidAggregatedUUID = (uuid: string) => {
    const uuidRegex =
      /^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;
    return uuidRegex.test(uuid);
  };

  const validateJsonInput = (jsonString: string) => {
    setJsonError("");
    setParsedData(null);

    if (!jsonString.trim()) {
      return;
    }

    try {
      const parsed = JSON.parse(jsonString);

      // Check if all required fields are present
      if (
        !parsed.publicKey ||
        !parsed.networkAddress ||
        !parsed.peerId ||
        (role !== EnumPlayerRole.Player1 && !parsed.aggregatedId)
      ) {
        setJsonError(
          "Missing required fields. Expected: publicKey, networkAddress, peerId" +
            (role !== EnumPlayerRole.Player1 ? ", aggregatedId" : "")
        );
        return;
      }

      // Validate each field
      const errors: string[] = [];

      if (
        role !== EnumPlayerRole.Player1 &&
        !isValidAggregatedUUID(parsed.aggregatedId)
      ) {
        errors.push("Invalid aggregatedId format (must be a valid UUID)");
      }

      if (!isValidPubKey(parsed.publicKey)) {
        errors.push("Invalid publicKey format (must be 66 hex characters)");
      }

      if (!isValidNetworkAddress(parsed.networkAddress)) {
        errors.push(
          "Invalid networkAddress format (e.g., /ip4/127.0.0.1/tcp/61181)"
        );
      }

      if (!isValidPeeId(parsed.peerId)) {
        errors.push("Invalid peerId format (must be hex characters)");
      }

      if (errors.length > 0) {
        setJsonError(errors.join(". "));
        return;
      }

      setParsedData(parsed);
    } catch {
      setJsonError("Invalid JSON format");
    }
  };

  const handleJsonChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    const value = e.target.value;
    setJsonInput(value);
    validateJsonInput(value);
  };

  const handleSetConnection = () => {
    if (
      !parsedData ||
      (!parsedData.aggregatedId && role !== EnumPlayerRole.Player1)
    )
      return;

    const operator_keys =
      role === EnumPlayerRole.Player2
        ? [operatorKey?.pub_key ?? "", parsedData.publicKey]
        : [parsedData.publicKey, operatorKey?.pub_key ?? ""];

    const participants_addresses =
      role === EnumPlayerRole.Player2
        ? [
            {
              address: peerConnectionInfo?.address ?? "",
              peer_id: peerConnectionInfo?.peer_id ?? "",
            },
            { address: parsedData.networkAddress, peer_id: parsedData.peerId },
          ]
        : [
            { address: parsedData.networkAddress, peer_id: parsedData.peerId },
            {
              address: peerConnectionInfo?.address ?? "",
              peer_id: peerConnectionInfo?.peer_id ?? "",
            },
          ];

    savePeerConnection({
      role,
      participants_addresses,
      operator_keys,
      aggregated_id:
        role === EnumPlayerRole.Player1
          ? aggregatedId
          : parsedData.aggregatedId ?? "",
    });
    setInputsDisabled(true);

    if (isSavingParticipantInfo) {
      setSuccessMessage(false);
    } else {
      setSuccessMessage(true);
    }
  };

  let aggregatedIdPlaceholder = "";
  if (role !== EnumPlayerRole.Player1) {
    aggregatedIdPlaceholder = `,\n "aggregatedId": "7d305b69-947e-4c90-a152-60365e47dc00"`;
  }

  const jsonPlaceholder = `{
  "publicKey": "0206b7b...87d31b7d",
  "networkAddress": "/ip4/127.0.0.1/tcp/61180",
  "peerId": "3082012....203010001${aggregatedIdPlaceholder}"
}`;

  return (
    <div className="p-4 bg-white border border-gray-200 rounded-lg">
      <Collapsible open={isOpen} onOpenChange={setIsOpen}>
        <CollapsibleTrigger asChild>
          <h3 className="font-semibold mb-3 text-gray-800 cursor-pointer hover:text-gray-900">
            🔗 Other Player&apos;s Participant Data
          </h3>
        </CollapsibleTrigger>
        <CollapsibleContent className="flex flex-col gap-3">
          <p className="text-sm text-gray-700">
            {role === EnumPlayerRole.Player1
              ? "Paste the JSON data containing the Public Key, Network Address and Peer ID of the other player to connect to your game."
              : "Paste the JSON data containing the Aggregated UUID, Public Key, Network Address and Peer ID of the other player to join their game."}
          </p>

          <p className="text-sm text-gray-700 mb-3">
            BitVMX will use all this information to connect to the other client
            and start computing the program.
          </p>

          <Textarea
            id="jsonInput"
            value={jsonInput}
            onChange={handleJsonChange}
            placeholder={jsonPlaceholder}
            className="font-mono text-sm min-h-[200px] resize-vertical"
            rows={8}
            disabled={inputsDisabled}
          />
          {jsonError && (
            <p className="text-red-600 text-sm mt-1">{jsonError}</p>
          )}

          <Button
            onClick={handleSetConnection}
            disabled={!parsedData || !!jsonError || inputsDisabled}
            className="w-full bg-gray-600 hover:bg-gray-700"
          >
            {isSavingParticipantInfo
              ? "⏳ Setting up Data..."
              : "🔗 Setup Data"}
          </Button>

          {!successMessage && (
            <div className="p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
              <h3 className="font-semibold mb-2 text-yellow-800">
                ⚠️ Connection Setup Required
              </h3>
              <p className="text-sm text-yellow-700">
                Paste the JSON data containing the other player&apos;s
                connection information to finalize the connection setup.
              </p>
            </div>
          )}
        </CollapsibleContent>
      </Collapsible>
    </div>
  );
}
