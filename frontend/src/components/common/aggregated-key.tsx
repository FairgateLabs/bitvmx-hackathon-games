import { useState } from "react";
import { Label } from "@/components/ui/label";
import { CopyButton } from "@/components/ui/copy-button";
import { useGetParticipantInfo } from "@/hooks/useParticipantInfo";
import { useCurrentGame } from "@/hooks/useGame";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";

export function AggregatedKey() {
  const [isOpen, setIsOpen] = useState(true);
  const {
    data: currentGame,
    isLoading: isGameLoading,
    error: gameError,
  } = useCurrentGame();

  const {
    data: aggregatedKeyData,
    isLoading: isKeyLoading,
    error: keyError,
  } = useGetParticipantInfo(currentGame?.id ?? "");

  if (isGameLoading || isKeyLoading) {
    return (
      <div className="p-4 bg-white border border-gray-200 rounded-lg">
        <p className="text-sm text-gray-600">Loading aggregated key...</p>
      </div>
    );
  }

  if (gameError || keyError) {
    return (
      <div className="p-4 bg-white border border-red-200 rounded-lg">
        <p className="text-sm text-red-600">
          Error loading aggregated key:{" "}
          {gameError?.message || keyError?.message}
        </p>
      </div>
    );
  }

  return (
    <div className="p-4 bg-white border border-gray-200 rounded-lg">
      <Collapsible open={isOpen} onOpenChange={setIsOpen}>
        <CollapsibleTrigger asChild>
          <h3 className="font-semibold mb-3 text-gray-800 cursor-pointer hover:text-gray-900">
            🔑 Aggregated Key
          </h3>
        </CollapsibleTrigger>
        <CollapsibleContent>
          <div className="mb-4 p-3 bg-blue-50 border border-blue-200 rounded-lg">
            <p className="text-sm text-blue-800 font-medium">
              BitVMX has generated an aggregated key between both participants
              (Player 1 and Player 2). This aggregated key will sign the whole
              protocol.
            </p>
          </div>

          <div className="space-y-3">
            <div className="flex items-center justify-between">
              <div className="flex-1">
                <Label className="text-gray-800">Aggregated Key:</Label>
                <p className="font-mono text-sm bg-gray-100 p-2 rounded overflow-hidden text-ellipsis whitespace-nowrap max-w-[500px]">
                  {aggregatedKeyData?.aggregated_key || "N/A"}
                </p>
              </div>
              <CopyButton
                text={aggregatedKeyData?.aggregated_key ?? ""}
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
