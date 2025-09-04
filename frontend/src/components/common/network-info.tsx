import { useState } from "react";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { useNetwork } from "@/hooks/useNetwork";

export function NetworkInfo() {
  const [isOpen, setIsOpen] = useState(true);
  const { data: network } = useNetwork();

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
            <span className="text-sm font-semibold">{network}</span> network.
          </p>
        </CollapsibleContent>
      </Collapsible>
    </div>
  );
}
