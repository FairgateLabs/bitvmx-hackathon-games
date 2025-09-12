import { useState } from "react";
import { Label } from "@/components/ui/label";
import { Button } from "@/components/ui/button";
import { useCommunicationInfo } from "@/hooks/useCommunicationInfo";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import usePubkey from "@/hooks/usePubkey";
import { EnumPlayerRole } from "@/types/game";
import { PlayerRole } from "../../../../backend/bindings/PlayerRole";

export function PeerConnectionInfo({
  aggregatedId,
  role,
  expanded,
}: {
  aggregatedId: string;
  role: PlayerRole;
  expanded: boolean;
}) {
  const [isOpen, setIsOpen] = useState(expanded);
  const { data: peerConnectionInfo } = useCommunicationInfo();
  const { data: operatorKey } = usePubkey();

  const handleCopyAllData = () => {
    if (!operatorKey || !peerConnectionInfo) return;

    const dataToCopy = {
      aggregatedId: aggregatedId,
      publicKey: operatorKey?.pub_key,
      networkAddress: peerConnectionInfo?.address,
      peerId: peerConnectionInfo?.peer_id,
    };

    navigator.clipboard.writeText(JSON.stringify(dataToCopy, null, 2));
  };

  return (
    <div className="p-4 bg-white border border-gray-200 rounded-lg">
      <Collapsible open={isOpen} onOpenChange={setIsOpen}>
        <CollapsibleTrigger asChild>
          <div className="flex justify-between items-center">
            <h3 className="font-semibold mb-3 text-gray-800 cursor-pointer hover:text-gray-900">
              🔌 Your Participant Data
            </h3>
          </div>
        </CollapsibleTrigger>
        <CollapsibleContent>
          <div className="flex justify-between items-center">
            {role === EnumPlayerRole.Player1 && (
              <p className="text-sm text-gray-700 mb-4">
                Share this information with Player 2 to enable them to connect
                with you and join your game.
              </p>
            )}
            {role === EnumPlayerRole.Player2 && (
              <p className="text-sm text-gray-700 mb-4">
                Share this information with Player 1 to enable them to connect
                with you.
              </p>
            )}
            <div className="flex justify-end">
              <Button onClick={handleCopyAllData} size="sm" variant="outline">
                Copy to Share
              </Button>
            </div>
          </div>

          <div className="space-y-3">
            {aggregatedId && (
              <div>
                <Label className="text-gray-800">Aggregated ID:</Label>
                <p className="font-mono text-sm bg-gray-100 p-2 rounded overflow-hidden text-ellipsis whitespace-nowrap max-w-[500px]">
                  {aggregatedId}
                </p>
              </div>
            )}

            <div>
              <Label className="text-gray-800">Public Key:</Label>
              <p className="font-mono text-sm bg-gray-100 p-2 rounded overflow-hidden text-ellipsis whitespace-nowrap max-w-[500px]">
                {operatorKey?.pub_key || "Loading..."}
              </p>
            </div>
            <div>
              <Label className="text-gray-800">Your Network Address:</Label>
              <p className="font-mono text-sm bg-gray-100 p-2 rounded overflow-hidden text-ellipsis whitespace-nowrap max-w-[500px]">
                {peerConnectionInfo?.address || "Loading..."}
              </p>
            </div>

            <div>
              <Label className="text-gray-800">Peer ID:</Label>
              <p className="font-mono text-sm bg-gray-100 p-2 rounded overflow-hidden text-ellipsis whitespace-nowrap max-w-[500px]">
                {peerConnectionInfo?.peer_id || "Loading..."}
              </p>
            </div>
          </div>
        </CollapsibleContent>
      </Collapsible>
    </div>
  );
}
