import { useState } from "react";
import { Label } from "@/components/ui/label";
import { Button } from "@/components/ui/button";
import { NetworkType } from "@/types/network";

interface WalletInfo {
  address: string;
  balance?: number;
}

interface WalletSectionProps {
  walletInfo: WalletInfo;
  networkSelected: NetworkType;
}

export function WalletSection({
  walletInfo,
  networkSelected,
}: WalletSectionProps) {
  const [transactionId, setTransactionId] = useState("");
  const [outputIndex, setOutputIndex] = useState("");

  if (!walletInfo) return null;

  return (
    <div className="p-4 bg-muted/50 rounded-lg">
      <h3 className="font-semibold mb-2">💰 Wallet Information</h3>
      <div className="grid grid-cols-2 gap-4 text-sm">
        <div>
          <Label>Address:</Label>
          <p className="font-mono text-xs break-all">{walletInfo.address}</p>
        </div>
        <div>
          <Label>Balance:</Label>
          {networkSelected === NetworkType.Testnet && !walletInfo.balance ? (
            <p className="text-xs">
              Please ensure your balance includes at least some amount of
              Testnet Bitcoins.
            </p>
          ) : (
            <p className="font-semibold">{walletInfo.balance} BTC</p>
          )}
        </div>
      </div>

      {networkSelected === NetworkType.Testnet && (
        <div className="mt-4">
          <p className="text-sm mb-2">
            Please fund the address provided. Once the transaction is on-chain
            and mined, provide the transaction ID and U below.
          </p>
          <div className="mb-2">
            <Label>Transaction ID:</Label>
            <input
              type="text"
              value={transactionId}
              onChange={(e) => setTransactionId(e.target.value)}
              className="w-full p-2 border rounded"
              placeholder="Enter transaction ID"
            />
          </div>
          <div className="mb-2">
            <Label>Output index:</Label>
            <input
              type="text"
              value={outputIndex}
              onChange={(e) => setOutputIndex(e.target.value)}
              className="w-full p-2 border rounded"
              placeholder="Enter Output index"
            />
          </div>
          <Button
            className="mt-2"
            onClick={() => alert("Done, I already did it!")}
          >
            Done, I already did it
          </Button>
        </div>
      )}
    </div>
  );
}
