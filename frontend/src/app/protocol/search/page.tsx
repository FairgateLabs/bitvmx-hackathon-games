"use client";
import { Input } from "@/components/ui/input";
import { useEffect, useState } from "react";
import useProtocol from "@/hooks/useProtocol";

export default function ProtocolSearch() {
  const [protocolId, setProtocolId] = useState<string>("");
  const { data: protocol, isLoading } = useProtocol(protocolId);

  useEffect(() => {
    const searchParams = new URLSearchParams(window.location.search);
    const id = searchParams.get("id");
    if (id) {
      setProtocolId(id);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <div className="flex flex-col min-h-screen p-8 pb-20 gap-8 sm:p-20 font-[family-name:var(--font-geist-sans)] overflow-scroll">
      <h1 className="text-4xl font-bold">Search Protocol</h1>
      <Input
        type="text"
        value={protocolId}
        onChange={(e) => setProtocolId(e.target.value)}
        placeholder="Enter Protocol ID"
      />
      {isLoading && <div className="text-gray-500">Loading...</div>}
      {!isLoading && !protocol && protocolId && (
        <div className="text-red-500">Protocol not found!</div>
      )}
      {protocol && (
        <div className="flex flex-col gap-4 mt-4">
          <div className="font-bold">Protocol Graph</div>
          <div className="font-mono text-xs bg-gray-100 p-3 rounded overflow-x-auto">
            {protocol && (
              <div
                className="w-full overflow-x-auto"
                dangerouslySetInnerHTML={{ __html: protocol.outerHTML }}
              />
            )}
          </div>
        </div>
      )}
    </div>
  );
}
