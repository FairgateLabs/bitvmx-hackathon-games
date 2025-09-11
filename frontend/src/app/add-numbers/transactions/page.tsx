"use client";

import { useState } from "react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { ExternalLink, Eye, EyeOff } from "lucide-react";
import { CopyButton } from "@/components/ui/copy-button";

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

const hardcodedTransactions: BitcoinTransaction[] = [
  {
    id: "1",
    hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
    blockHeight: 123456,
    confirmations: 6,
    fee: 0.0001,
    inputs: 2,
    outputs: 1,
    totalInput: 0.5,
    totalOutput: 0.4999,
    timestamp: "2024-01-15T10:30:00Z",
    jsonData: {
      version: 1,
      locktime: 0,
      vin: [
        {
          txid: "0xa1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef1234567890abcdef",
          vout: 0,
          scriptSig: {
            asm: "304402207f8b9c0d1e2f3456789012345678901234567890abcdef1234567890abcdef1234567890abcdef022100a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef1234567890abcdef[ALL] 02a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef1234567890abcdef",
            hex: "47304402207f8b9c0d1e2f3456789012345678901234567890abcdef1234567890abcdef1234567890abcdef022100a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef1234567890abcdef012102a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef1234567890abcdef",
          },
          sequence: 4294967295,
        },
      ],
      vout: [
        {
          value: 0.4999,
          n: 0,
          scriptPubKey: {
            asm: "OP_DUP OP_HASH160 a1b2c3d4e5f6789012345678901234567890abcdef OP_EQUALVERIFY OP_CHECKSIG",
            hex: "76a914a1b2c3d4e5f6789012345678901234567890abcdef88ac",
            reqSigs: 1,
            type: "pubkeyhash",
            addresses: ["1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"],
          },
        },
      ],
    },
  },
  {
    id: "2",
    hash: "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
    blockHeight: 123457,
    confirmations: 3,
    fee: 0.00015,
    inputs: 1,
    outputs: 2,
    totalInput: 1.0,
    totalOutput: 0.99985,
    timestamp: "2024-01-15T11:45:00Z",
    jsonData: {
      version: 1,
      locktime: 0,
      vin: [
        {
          txid: "0xb2c3d4e5f6789012345678901234567890abcdef1234567890abcdef1234567890abcdef",
          vout: 0,
          scriptSig: {
            asm: "304402207f8b9c0d1e2f3456789012345678901234567890abcdef1234567890abcdef1234567890abcdef022100b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef1234567890abcdef[ALL] 02b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef1234567890abcdef",
            hex: "47304402207f8b9c0d1e2f3456789012345678901234567890abcdef1234567890abcdef1234567890abcdef022100b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef1234567890abcdef012102b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef1234567890abcdef",
          },
          sequence: 4294967295,
        },
      ],
      vout: [
        {
          value: 0.5,
          n: 0,
          scriptPubKey: {
            asm: "OP_DUP OP_HASH160 b2c3d4e5f6789012345678901234567890abcdef OP_EQUALVERIFY OP_CHECKSIG",
            hex: "76a914b2c3d4e5f6789012345678901234567890abcdef88ac",
            reqSigs: 1,
            type: "pubkeyhash",
            addresses: ["1B2zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"],
          },
        },
        {
          value: 0.49985,
          n: 1,
          scriptPubKey: {
            asm: "OP_DUP OP_HASH160 c3d4e5f6789012345678901234567890abcdef123456 OP_EQUALVERIFY OP_CHECKSIG",
            hex: "76a914c3d4e5f6789012345678901234567890abcdef12345688ac",
            reqSigs: 1,
            type: "pubkeyhash",
            addresses: ["1C3zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"],
          },
        },
      ],
    },
  },
  {
    id: "3",
    hash: "0x567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234",
    blockHeight: 123458,
    confirmations: 1,
    fee: 0.00008,
    inputs: 1,
    outputs: 1,
    totalInput: 0.25,
    totalOutput: 0.24992,
    timestamp: "2024-01-15T12:15:00Z",
    jsonData: {
      version: 1,
      locktime: 0,
      vin: [
        {
          txid: "0xc3d4e5f6789012345678901234567890abcdef1234567890abcdef1234567890abcdef",
          vout: 0,
          scriptSig: {
            asm: "304402207f8b9c0d1e2f3456789012345678901234567890abcdef1234567890abcdef1234567890abcdef022100c3d4e5f6789012345678901234567890abcdef1234567890abcdef1234567890abcdef[ALL] 02c3d4e5f6789012345678901234567890abcdef1234567890abcdef1234567890abcdef",
            hex: "47304402207f8b9c0d1e2f3456789012345678901234567890abcdef1234567890abcdef1234567890abcdef022100c3d4e5f6789012345678901234567890abcdef1234567890abcdef1234567890abcdef012102c3d4e5f6789012345678901234567890abcdef1234567890abcdef1234567890abcdef",
          },
          sequence: 4294967295,
        },
      ],
      vout: [
        {
          value: 0.24992,
          n: 0,
          scriptPubKey: {
            asm: "OP_DUP OP_HASH160 d4e5f6789012345678901234567890abcdef1234567890abcdef OP_EQUALVERIFY OP_CHECKSIG",
            hex: "76a914d4e5f6789012345678901234567890abcdef1234567890abcdef88ac",
            reqSigs: 1,
            type: "pubkeyhash",
            addresses: ["1D4zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"],
          },
        },
      ],
    },
  },
];

export default function TransactionList() {
  const [showJson, setShowJson] = useState<{ [key: string]: boolean }>({});

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

  const openExplorer = (hash: string) => {
    // For now, using a mock explorer URL - replace with actual Bitcoin explorer
    const explorerUrl = `https://blockstream.info/tx/${hash}`;
    window.open(explorerUrl, "_blank");
  };

  return (
    <div className="container mx-auto p-6 max-w-6xl">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">
          Transactions for game Add Numbers
        </h1>
        <p className="text-gray-600">
          Recent transactions from the Add Numbers game
        </p>
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
                    Block {tx.blockHeight} • {tx.confirmations} confirmations
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
                      onClick={() => openExplorer(tx.hash)}
                    >
                      <ExternalLink className="h-3 w-3" />
                    </Button>
                  </div>
                </div>

                <div className="space-y-1">
                  <div className="text-sm font-medium text-gray-500">Fee</div>
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
  );
}
