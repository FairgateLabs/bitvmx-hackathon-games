import { Loader } from "../ui/loader";

interface DisputeTransaction {
  tx_id: string;
  block_info?: {
    height?: number;
    hash?: string;
  };
}

interface ChallengeStatusDisplayProps {
  transactions: { [key: string]: DisputeTransaction };
}

export function ChallengeStatusDisplay({
  transactions,
}: ChallengeStatusDisplayProps) {
  const formatHash = (hash: string) => {
    return `${hash.substring(0, 4)}...${hash.substring(hash.length - 4)}`;
  };

  // Sort transactions by block height (descending order - highest first)
  const sortedTransactions = Object.entries(transactions).sort(
    ([, a], [, b]) => {
      const blockHeightA = a?.block_info?.height || 0;
      const blockHeightB = b?.block_info?.height || 0;
      return blockHeightB - blockHeightA;
    }
  );

  return (
    <div>
      <div className="text-xl text-gray-800 flex items-center gap-2">
        ⚡ Challenge Initiated - On-Chain Verification
        <Loader />
      </div>
      <div className="text-gray-700 mt-4 mb-4">
        The dispute resolution has been activated. Both players must now
        participate in the on-chain verification process to determine the winner
        and claim the bet funds. This process is automatically run by the BitVMX
        protocol and will take some time.
      </div>
      <div className="space-y-4">
        <div className="p-4 bg-white rounded-lg border border-gray-200">
          <h3 className="font-semibold text-gray-800 mb-3">
            🔗 Active Dispute Transactions
          </h3>
          <p className="text-sm text-gray-600 mb-4">
            The following transactions have been submitted to the Bitcoin
            network as part of the BitVMX dispute resolution protocol:
          </p>
          <div className="space-y-2">
            {sortedTransactions.map(([key, txData]) => (
              <div
                key={key}
                className="flex items-center justify-between p-3 bg-gray-50 rounded border"
              >
                <div className="flex items-center gap-3">
                  <span className="font-mono text-sm font-medium text-gray-700">
                    {key}
                  </span>
                  <span className="text-gray-400">→</span>
                  <span className="font-mono text-sm text-gray-600">
                    {txData?.tx_id ? formatHash(txData.tx_id) : "N/A"}
                  </span>
                  {txData?.block_info?.height && (
                    <span className="text-xs text-gray-500">
                      (Block: {txData.block_info.height})
                    </span>
                  )}
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
