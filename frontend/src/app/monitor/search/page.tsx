"use client";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { useEffect, useState } from "react";
import { getMonitorByString, MonitorType } from "@/app/helper";
import { TransactionLink } from "@/components/ui/transaction-link";
import useMonitor from "@/hooks/useMonitor";

export default function TransactionSearch() {
  const [monitorType, setMonitorType] = useState<MonitorType | null>(null);
  const [monitorTxid, setMonitorTxid] = useState<string>("");

  const { data: monitor, isLoading: isMonitorLoading } = useMonitor(
    monitorType as MonitorType,
    monitorTxid
  );

  useEffect(() => {
    const searchParams = new URLSearchParams(window.location.search);
    const monitorType = searchParams.get("monitor_type");
    const transactionId = searchParams.get("txid");

    if (
      monitorType === MonitorType.Transaction ||
      monitorType === MonitorType.SpendingUTXOTransaction
    ) {
      setMonitorType(monitorType);
      setMonitorTxid(transactionId as string);
    }

    if (monitorType === MonitorType.RskPeginTransaction) {
      setMonitorType(monitorType);
    }
  }, []);

  return (
    <div className="flex flex-col min-h-screen p-8 pb-20 gap-8 sm:p-20 font-[family-name:var(--font-geist-sans)]">
      <h1 className="text-4xl font-bold">Search Monitor</h1>
      <div className="flex items-center gap-4 max-w-[600px]">
        <Select
          value={monitorType || ""}
          onValueChange={(value) => setMonitorType(getMonitorByString(value))}
        >
          <SelectTrigger className="w-[250px]">
            <SelectValue placeholder="Choose monitor type" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="Transaction">Transaction</SelectItem>
            <SelectItem value="SpendingUTXOTransaction">
              Spending UTXO Transaction
            </SelectItem>
            <SelectItem value="RskPeginTransaction">
              RSK Pegin Transaction
            </SelectItem>
          </SelectContent>
        </Select>

        <Input
          type="text"
          value={monitorTxid}
          onChange={(e) => setMonitorTxid(e.target.value)}
          placeholder="Enter Monitor ID"
        />
        <Button
          onClick={(e) => {
            if (monitorType) {
              setMonitorType(monitorType);
              setMonitorTxid(monitorTxid);
            }
          }}
        >
          Search
        </Button>
      </div>

      {monitor && (
        <div>
          {monitor.Transaction && (
            <div className="flex flex-col gap-2">
              <div className="flex gap-2">
                <span className="font-semibold min-w-[140px]">
                  Transaction ID:
                </span>
                <span className="font-mono break-all">
                  <TransactionLink txId={monitor.Transaction?.[0] ?? ""} />
                </span>
              </div>
              <div className="flex gap-2">
                <span className="font-semibold min-w-[140px]">
                  Context Data:
                </span>
                <pre className="overflow-x-auto flex-1">
                  {JSON.stringify(monitor.Transaction?.[1], null, 2)}
                </pre>
              </div>
            </div>
          )}

          {monitor.SpendingUTXOTransaction && (
            <div className="flex flex-col gap-2">
              <div className="flex gap-2">
                <span className="font-semibold min-w-[140px]">
                  Transaction ID:
                </span>
                <span className="font-mono break-all">
                  <TransactionLink
                    txId={monitor.SpendingUTXOTransaction?.[0] ?? ""}
                  />
                </span>
              </div>
              <div className="flex gap-2">
                <span className="font-semibold min-w-[140px]">VOUT:</span>
                <span>{monitor.SpendingUTXOTransaction?.[1]}</span>
              </div>
              <div className="flex gap-2">
                <span className="font-semibold min-w-[140px]">
                  Context Data:
                </span>
                <pre className="overflow-x-auto flex-1">
                  {JSON.stringify(
                    monitor.SpendingUTXOTransaction?.[2],
                    null,
                    2
                  )}
                </pre>
              </div>
            </div>
          )}

          {monitor.RskPeginTransaction && (
            <div className="flex flex-col gap-2">
              <div className="flex gap-2">
                <span className="font-semibold min-w-[140px]">
                  Transaction ID:
                </span>
                <span className="font-mono break-all">
                  <TransactionLink
                    txId={monitor.RskPeginTransaction?.[0] ?? ""}
                  />
                </span>
              </div>
              <div className="flex gap-2">
                <span className="font-semibold min-w-[140px]">
                  RSK Pegin Transaction Data:
                </span>
                <span>
                  {/* You can expand this to show more details if available */}
                  {JSON.stringify(monitor.RskPeginTransaction?.[1], null, 2)}
                </span>
              </div>
            </div>
          )}
        </div>
      )}

      {isMonitorLoading && <div className="text-gray-500">Loading...</div>}

      {!isMonitorLoading && !monitor && monitorTxid && monitorType && (
        <div className="text-red-500">Monitor ID not found!</div>
      )}
    </div>
  );
}
