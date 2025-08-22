"use client";
import { Input } from "@/components/ui/input";
import useTransaction from "@/hooks/useTransaction";
import { useEffect, useState } from "react";

export default function TransactionSearch() {
  const [searchId, setSearchId] = useState<string>("");
  const { data: transactionInfo, isLoading } = useTransaction(searchId);

  useEffect(() => {
    const searchParams = new URLSearchParams(window.location.search);
    const id = searchParams.get("id");

    if (id) {
      setSearchId(id);
    }
  }, [searchId]);

  return (
    <div className="flex flex-col min-h-screen p-8 pb-20 gap-8 sm:p-20 font-[family-name:var(--font-geist-sans)]">
      <h1 className="text-4xl font-bold">Search Transaction</h1>
      <Input
        type="text"
        value={searchId}
        onChange={(e) => setSearchId(e.target.value)}
        placeholder="Enter Transaction ID"
      />
      {transactionInfo && (
        <div className="flex flex-col gap-4">
          <div className="flex gap-4">
            <span className="font-bold">Block Height:</span>
            <span className="font-mono">{transactionInfo.block_height}</span>
          </div>
          <div className="flex gap-4">
            <span className="font-bold">Block Hash:</span>
            <span className="font-mono">{transactionInfo.block_hash}</span>
          </div>
          <div className="flex gap-4">
            <span className="font-bold">Orphan:</span>
            <span className="font-mono">
              {transactionInfo.orphan ? "Yes" : "No"}
            </span>
          </div>
          <div className="flex gap-4">
            <span className="font-bold">Confirmations:</span>
            <span className="font-mono">{transactionInfo.confirmations}</span>
          </div>

          <div className="flex flex-col gap-2">
            <span className="font-bold">Transaction Data:</span>
            <pre className="bg-gray-100 p-4 rounded overflow-auto">
              {JSON.stringify(transactionInfo.tx, null, 2)}
            </pre>
          </div>
        </div>
      )}

      {isLoading && <div className="text-gray-500">Loading...</div>}
    </div>
  );
}
