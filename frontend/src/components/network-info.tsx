import { useState } from "react";
import { NetworkType } from "@/types/network";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";

interface NetworkInfoProps {
  networkSelected: NetworkType | null;
}

export function NetworkInfo({ networkSelected }: NetworkInfoProps) {
  const [isOpen, setIsOpen] = useState(true);

  if (!networkSelected) return null;

  return (
    <div className="p-4 bg-white border border-gray-200 rounded-lg">
      <Collapsible open={isOpen} onOpenChange={setIsOpen}>
        <CollapsibleTrigger asChild>
          <h3 className="font-semibold mb-2 cursor-pointer hover:text-gray-900">
            🔗 Network Information
          </h3>
        </CollapsibleTrigger>
        <CollapsibleContent>
          <p className="text-sm text-gray-700">
            You are currently connected to the{" "}
            <span className="text-sm font-semibold">{networkSelected}</span>{" "}
            network.
          </p>
        </CollapsibleContent>
      </Collapsible>
    </div>
  );
}
