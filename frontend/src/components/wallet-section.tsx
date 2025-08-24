import { Label } from "@/components/ui/label";

interface WalletInfo {
  address: string;
  balance: number;
  network: "regtest" | "testnet";
}

interface WalletSectionProps {
  walletInfo: WalletInfo | null;
}

export function WalletSection({ walletInfo }: WalletSectionProps) {
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
          <p className="font-semibold">{walletInfo.balance} BTC</p>
        </div>
      </div>
    </div>
  );
}
