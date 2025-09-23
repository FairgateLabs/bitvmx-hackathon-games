"use client";

import { useEffect, useState } from "react";
import { BackendStatus } from "@/components/common/backend-status";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { ExternalLink, Eye, EyeOff, BarChart3 } from "lucide-react";
import { CopyButton } from "@/components/ui/copy-button";
import { useProtocolVisualization } from "@/hooks/useProtocolVisualization";
import { useCurrentGame } from "@/hooks/useGame";
import { ProtocolVisualizationPopup } from "@/components/common/protocol-visualization-popup";
import { highlightNode } from "@/visualize";
import { instance } from "@viz-js/viz";

type TransactionInput = {
  previous_output: string;
  script_sig: string;
  sequence: number;
  witness: string[];
};

type TransactionOutput = {
  script_pubkey: string;
  value: number;
};

type Transaction = {
  input: TransactionInput[];
  lock_time: number;
  output: TransactionOutput[];
  version: number;
};

type DisputeTransaction = {
  tx: Transaction;
  tx_id: string;
};

type Transactions = {
  [key: string]: DisputeTransaction;
};

export default function TransactionList() {
  const [showJson, setShowJson] = useState<{ [key: string]: boolean }>({});
  const [isProtocolPopupOpen, setIsProtocolPopupOpen] = useState(false);
  const [protocolVisualization, setProtocolVisualization] =
    useState<SVGSVGElement | null>(null);
  const { data: currentGame } = useCurrentGame();
  const toggleJson = (txId: string) => {
    setShowJson((prev) => ({
      ...prev,
      [txId]: !prev[txId],
    }));
  };

  const formatHash = (hash: string) => {
    return `${hash.substring(0, 8)}...${hash.substring(hash.length - 8)}`;
  };

  const getLink = (hash: string) => {
    return `http://localhost:4000/tx/${hash}`;
  };

  const openExplorer = (hash: string) => {
    const explorerUrl = getLink(hash);
    window.open(explorerUrl, "_blank");
  };

  const { data: protocol } = useProtocolVisualization(
    currentGame?.program_id ?? ""
  );

  const transactions = currentGame?.bitvmx_program_properties
    .dispute_tx as Transactions;

  useEffect(() => {
    const highlightDisputeTxs = async () => {
      if (!protocol) return;
      const disputeTxKeys = Object.keys(
        currentGame?.bitvmx_program_properties.dispute_tx || {}
      );
      let protocolVisualization = protocol;
      const inst = await instance();

      for (const key of disputeTxKeys) {
        const data = currentGame?.bitvmx_program_properties.dispute_tx?.[
          key
        ] as {
          tx_id: string;
        };
        const url = getLink(data.tx_id);
        protocolVisualization = highlightNode(protocolVisualization, key, url);
      }
      const svg = inst.renderSVGElement(protocolVisualization as string);
      setProtocolVisualization(svg);
    };
    highlightDisputeTxs();
  }, [currentGame?.bitvmx_program_properties.dispute_tx, protocol]);

  return (
    <BackendStatus>
      <>
        {!currentGame?.bitvmx_program_properties.dispute_tx && (
          <div className="container mx-auto p-6 max-w-4xl">
            <div className="text-center">No transactions found!</div>
          </div>
        )}
        {currentGame?.program_id &&
          currentGame?.bitvmx_program_properties.dispute_tx && (
            <div className="container mx-auto p-6 max-w-4xl">
              <div className="flex items-center justify-between mb-4">
                <div>
                  <h1 className="text-3xl font-bold text-gray-900 mb-2">
                    Transactions for game Add Numbers
                  </h1>
                  <p className="text-gray-600">
                    Recent transactions from the Add Numbers game
                  </p>
                </div>
                <Button
                  onClick={() => setIsProtocolPopupOpen(true)}
                  className="flex items-center gap-2"
                  variant="outline"
                >
                  <BarChart3 className="h-4 w-4" />
                  View Protocol Visualization
                </Button>
              </div>

              <div className="space-y-6">
                {transactions &&
                  Object.keys(transactions).map((key) => {
                    const txData = transactions[key]?.tx;
                    const txId = transactions[key]?.tx_id;

                    return (
                      <Card
                        key={txId}
                        className="hover:shadow-lg transition-shadow"
                      >
                        <CardHeader>
                          <div className="flex items-center justify-between">
                            <div>
                              <CardTitle className="text-lg font-mono text-gray-800">
                                Transaction
                              </CardTitle>
                              <CardDescription className="text-sm text-gray-600">
                                Name in Protocol: <strong>{key}</strong>
                              </CardDescription>
                            </div>
                            <div className="flex items-center gap-2">
                              <Button
                                variant="outline"
                                size="sm"
                                onClick={() => toggleJson(txId)}
                              >
                                {showJson[txId] ? (
                                  <>
                                    <EyeOff className="h-4 w-4 mr-2" />
                                    Hide JSON
                                  </>
                                ) : (
                                  <>
                                    <Eye className="h-4 w-4 mr-2" />
                                    Show JSON
                                  </>
                                )}
                              </Button>
                            </div>
                          </div>
                        </CardHeader>

                        <CardContent>
                          <div className="flex flex-row justify-between mb-4 w-full">
                            <div className="space-y-1">
                              <div className="text-sm font-medium text-gray-500">
                                Transaction Hash
                              </div>

                              <div className="flex items-center gap-2">
                                <div className="font-mono text-sm text-gray-800 break-all">
                                  {formatHash(txId)}
                                </div>
                                <CopyButton text={txId} />
                                <Button
                                  variant="outline"
                                  size="sm"
                                  onClick={() => openExplorer(txId)}
                                >
                                  <ExternalLink className="h-3 w-3" />
                                </Button>
                              </div>
                            </div>
                            <div>
                              <div className="text-sm font-medium text-gray-500">
                                Lock Time
                              </div>
                              <div className="text-sm text-gray-800">
                                {txData.lock_time}
                              </div>
                            </div>
                            <div className="space-y-1">
                              <div className="text-sm font-medium text-gray-500">
                                Inputs
                              </div>
                              <div className="text-sm text-gray-800">
                                {txData.input.length} input
                                {txData.input.length !== 1 ? "s" : ""}
                              </div>
                            </div>

                            <div className="space-y-1">
                              <div className="text-sm font-medium text-gray-500">
                                Outputs
                              </div>
                              <div className="text-sm text-gray-800">
                                {txData.output.length} output
                                {txData.output.length !== 1 ? "s" : ""}
                              </div>
                            </div>
                          </div>

                          {showJson[txId] && (
                            <div className="mt-4">
                              <div className="flex items-center justify-between mb-2">
                                <div className="text-sm font-medium text-gray-500">
                                  Transaction JSON
                                </div>
                                <CopyButton
                                  size="sm"
                                  variant="outline"
                                  text={JSON.stringify(txData, null, 2)}
                                >
                                  Copy JSON
                                </CopyButton>
                              </div>
                              <textarea
                                className="w-full h-64 p-3 bg-gray-50 border border-gray-200 rounded-md font-mono text-xs text-gray-700 resize-none"
                                value={JSON.stringify(txData, null, 2)}
                                readOnly
                              />
                            </div>
                          )}
                        </CardContent>
                      </Card>
                    );
                  })}
              </div>
            </div>
          )}

        <ProtocolVisualizationPopup
          isOpen={isProtocolPopupOpen}
          onClose={() => setIsProtocolPopupOpen(false)}
          visualization={protocolVisualization as unknown as SVGSVGElement}
          isLoading={false}
        />
      </>
    </BackendStatus>
  );
}
