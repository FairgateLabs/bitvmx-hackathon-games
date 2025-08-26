import { NetworkType } from "@/types/network";

interface NetworkInfoProps {
  networkSelected: NetworkType | null;
}

export function NetworkInfo({ networkSelected }: NetworkInfoProps) {
  if (!networkSelected) return null;

  return (
    <div className="p-4 bg-muted/50 rounded-lg">
      <h3 className="font-semibold mb-2">🔗 Network Information</h3>
      <p className="text-sm">
        You are currently connected to the{" "}
        <span className="text-sm font-semibold">{networkSelected}</span>{" "}
        network.
      </p>
    </div>
  );
}
