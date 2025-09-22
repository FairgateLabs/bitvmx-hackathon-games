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

interface BitcoinTransaction {
  id: string;
  hash: string;
  blockHeight: number;
  confirmations: number;
  fee: number;
  inputs: number;
  outputs: number;
  totalInput: number;
  totalOutput: number;
  timestamp: string;
  jsonData: Record<string, unknown>;
}

const hardcodedTransactions: BitcoinTransaction[] = [];

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

  const formatTimestamp = (timestamp: string) => {
    return new Date(timestamp).toLocaleString();
  };

  const formatBitcoin = (satoshi: number) => {
    return (satoshi / 100000000).toFixed(8);
  };

  const getLink = (hash: string) => {
    return `http://localhost:4000/tx/${hash}`;
  };

  const { data: protocol, isLoading: isProtocolLoading } =
    useProtocolVisualization(currentGame?.program_id ?? "");

  useEffect(() => {
    const highlightDisputeTxs = async () => {
      if (!protocol) return;
      const disputeTxKeys = Object.keys(
        currentGame?.bitvmx_program_properties.dispute_tx || {}
      );
      let protocolVisualization = protocol;
      let inst = await instance();

      for (const key of disputeTxKeys) {
        let data = currentGame?.bitvmx_program_properties.dispute_tx?.[key] as {
          tx_id: string;
        };
        let url = getLink(data.tx_id);
        protocolVisualization = highlightNode(protocolVisualization, key, url);
      }
      let svg = inst.renderSVGElement(protocolVisualization as string);
      setProtocolVisualization(svg);
    };
    highlightDisputeTxs();
  }, [currentGame?.bitvmx_program_properties.dispute_tx, protocol]);

  return (
    <BackendStatus>
      <>
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
            {/* {currentGame?.program_id && ( */}
            <Button
              onClick={() => setIsProtocolPopupOpen(true)}
              className="flex items-center gap-2"
              variant="outline"
            >
              <BarChart3 className="h-4 w-4" />
              View Protocol Visualization
            </Button>
            {/* )} */}
          </div>

          <div className="space-y-6">
            {hardcodedTransactions.map((tx) => (
              <Card key={tx.id} className="hover:shadow-lg transition-shadow">
                <CardHeader>
                  <div className="flex items-center justify-between">
                    <div>
                      <CardTitle className="text-lg font-mono text-gray-800">
                        Transaction #{tx.id}
                      </CardTitle>
                      <CardDescription className="text-sm text-gray-600">
                        Block {tx.blockHeight} • {tx.confirmations}{" "}
                        confirmations
                      </CardDescription>
                    </div>
                    <div className="flex items-center gap-2">
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => toggleJson(tx.id)}
                      >
                        {showJson[tx.id] ? (
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
                  <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 mb-4 w-full">
                    <div className="space-y-4 w-full">
                      <div className="text-sm font-medium text-gray-500">
                        Transaction Hash
                      </div>
                      <div className="flex items-center gap-2">
                        <div className="font-mono text-sm text-gray-800 break-all">
                          {formatHash(tx.hash)}
                        </div>
                        <CopyButton text={tx.hash} />
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => getLink(tx.hash)}
                        >
                          <ExternalLink className="h-3 w-3" />
                        </Button>
                      </div>
                    </div>

                    <div className="space-y-1">
                      <div className="text-sm font-medium text-gray-500">
                        Fee
                      </div>
                      <div className="text-sm text-gray-800">
                        {formatBitcoin(tx.fee * 100000000)} BTC
                      </div>
                    </div>

                    <div className="space-y-1">
                      <div className="text-sm font-medium text-gray-500">
                        Timestamp
                      </div>
                      <div className="text-sm text-gray-800">
                        {formatTimestamp(tx.timestamp)}
                      </div>
                    </div>
                  </div>

                  <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-4">
                    <div className="space-y-2">
                      <div className="text-sm font-medium text-gray-500">
                        Inputs
                      </div>
                      <div className="text-sm text-gray-800">
                        {tx.inputs} input{tx.inputs !== 1 ? "s" : ""} •{" "}
                        {formatBitcoin(tx.totalInput * 100000000)} BTC
                      </div>
                    </div>

                    <div className="space-y-2">
                      <div className="text-sm font-medium text-gray-500">
                        Outputs
                      </div>
                      <div className="text-sm text-gray-800">
                        {tx.outputs} output{tx.outputs !== 1 ? "s" : ""} •{" "}
                        {formatBitcoin(tx.totalOutput * 100000000)} BTC
                      </div>
                    </div>
                  </div>

                  {showJson[tx.id] && (
                    <div className="mt-4">
                      <div className="flex items-center justify-between mb-2">
                        <div className="text-sm font-medium text-gray-500">
                          Transaction JSON
                        </div>
                        <CopyButton
                          size="sm"
                          variant="outline"
                          text={JSON.stringify(tx.jsonData, null, 2)}
                        >
                          Copy JSON
                        </CopyButton>
                      </div>
                      <textarea
                        className="w-full h-64 p-3 bg-gray-50 border border-gray-200 rounded-md font-mono text-xs text-gray-700 resize-none"
                        value={JSON.stringify(tx.jsonData, null, 2)}
                        readOnly
                      />
                    </div>
                  )}
                </CardContent>
              </Card>
            ))}
          </div>
        </div>

        {/* Protocol Visualization Popup */}
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
