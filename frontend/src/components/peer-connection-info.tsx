import { useState, useEffect } from "react";
import { Label } from "@/components/ui/label";
import { CopyButton } from "@/components/ui/copy-button";
import { usePeerConnection } from "@/hooks/usePeerConnection";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";

export function PeerConnectionInfo() {
  const [isOpen, setIsOpen] = useState(true);
  const { data: peerConnectionInfo } = usePeerConnection();

  return (
    <div className="p-4 bg-white border border-gray-200 rounded-lg">
      <Collapsible open={isOpen} onOpenChange={setIsOpen}>
        <CollapsibleTrigger asChild>
          <h3 className="font-semibold mb-3 text-gray-800 cursor-pointer hover:text-gray-900">
            🔌 Your Connection Information
          </h3>
        </CollapsibleTrigger>
        <CollapsibleContent>
          <p className="text-sm text-gray-700 mb-4">
            Share this information with other players to enable them to connect
            to your game.
          </p>

          <div className="space-y-3">
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
                  {peerConnectionInfo?.peerId || "Loading..."}
                </p>
              </div>
              <CopyButton
                text={peerConnectionInfo?.peerId || "Loading..."}
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
