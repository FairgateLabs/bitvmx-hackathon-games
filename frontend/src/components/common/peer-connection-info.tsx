import { useState } from "react";
import { Label } from "@/components/ui/label";
import { CopyButton } from "@/components/ui/copy-button";
import { useCommunicationInfo } from "@/hooks/useCommunicationInfo";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import usePubkey from "@/hooks/usePubkey";
import { useGameRole } from "@/hooks/useGameRole";
import { PlayerRole } from "@/types/game";

export function PeerConnectionInfo({ gameId }: { gameId: string | null }) {
  const [isOpen, setIsOpen] = useState(true);
  const { data: peerConnectionInfo, isLoading, error } = useCommunicationInfo();
  const { data: operatorKey } = usePubkey();
  const { data: role } = useGameRole();

  return (
    <div className="p-4 bg-white border border-gray-200 rounded-lg">
      <Collapsible open={isOpen} onOpenChange={setIsOpen}>
        <CollapsibleTrigger asChild>
          <h3 className="font-semibold mb-3 text-gray-800 cursor-pointer hover:text-gray-900">
            🔌 Your Setup Data
          </h3>
        </CollapsibleTrigger>
        <CollapsibleContent>
          {role === PlayerRole.Player1 && (
            <p className="text-sm text-gray-700 mb-4">
              Share this information with Player 2 to enable them to connect
              with you and join your game.
            </p>
          )}
          {role === PlayerRole.Player2 && (
            <p className="text-sm text-gray-700 mb-4">
              Share this information with Player 1 to enable them to connect
              with you.
            </p>
          )}

          <div className="space-y-3">
            {gameId && (
              <div className="flex items-center justify-between">
                <div>
                  <Label className="text-gray-800">Game UUID:</Label>
                  <p className="font-mono text-sm bg-gray-100 p-2 rounded overflow-hidden text-ellipsis whitespace-nowrap max-w-[500px]">
                    {gameId}
                  </p>
                </div>
                <CopyButton text={gameId} size="sm" variant="outline" />
              </div>
            )}
            <div className="flex items-center justify-between">
              <div>
                <Label className="text-gray-800">Public Key:</Label>
                <p className="font-mono text-sm bg-gray-100 p-2 rounded overflow-hidden text-ellipsis whitespace-nowrap max-w-[500px]">
                  {operatorKey?.pub_key || "Loading..."}
                </p>
              </div>
              <CopyButton
                text={operatorKey?.pub_key ?? ""}
                size="sm"
                variant="outline"
              />
            </div>
            <div className="flex items-center justify-between">
              <div>
                <Label className="text-gray-800">Your Network Address:</Label>
                <p className="font-mono text-sm bg-gray-100 p-2 rounded">
                  {peerConnectionInfo?.address || "Loading..."}
                </p>
              </div>
              <div className="flex gap-2">
                <CopyButton
                  text={peerConnectionInfo?.address || ""}
                  size="sm"
                  variant="outline"
                />
              </div>
            </div>

            <div className="flex items-center justify-between">
              <div>
                <Label className="text-gray-800">Peer ID:</Label>
                <p className="font-mono text-sm bg-gray-100 p-2 rounded overflow-hidden text-ellipsis whitespace-nowrap max-w-[500px]">
                  {peerConnectionInfo?.peer_id || "Loading..."}
                </p>
              </div>
              <CopyButton
                text={peerConnectionInfo?.peer_id ?? ""}
                size="sm"
                variant="outline"
              />
            </div>
          </div>
        </CollapsibleContent>
      </Collapsible>
    </div>
  );
}
