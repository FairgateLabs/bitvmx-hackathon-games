"use client";
import { getMonitorTxid, getMonitorType, MonitorData } from "@/app/helper";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { TransactionLink } from "@/components/ui/transaction-link";
import useMonitorHeight from "@/hooks/useMonitorBlockHeight";
import useMonitors from "@/hooks/useMonitors";
import Link from "next/link";

export default function MonitorList() {
  const { data: monitors, isLoading: isMonitorsLoading } = useMonitors();
  const { data: monitorHeight, isLoading: isMonitorHeightLoading } =
    useMonitorHeight();

  return (
    <div className="flex  flex-col min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
      <h1 className="text-4xl font-bold">Monitor List</h1>

      <div className="flexjustify-end">
        <div className="flex items-center gap-2">
          <span className="text-gray-500">Monitor Block Height:</span>
          <span className="font-mono font-bold">
            {isMonitorHeightLoading && "Loading..."}
            {!isMonitorHeightLoading && monitorHeight}
          </span>
        </div>
      </div>

      {isMonitorsLoading && "Loading..."}
      {!isMonitorsLoading && (
        <Table className="">
          <TableHeader>
            <TableRow>
              <TableHead>Monitor Type</TableHead>
              <TableHead>Monitor Data</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {monitors.map((monitor: MonitorData, index: number) => (
              <TableRow key={index}>
                <TableCell className="font-mono font-bold text-left">
                  <Link
                    href={`/monitor/search?monitor_type=${getMonitorType(
                      monitor
                    )}&txid=${getMonitorTxid(monitor)}`}
                    className="hover:underline cursor-pointer"
                  >
                    {monitor.Transaction
                      ? "Transaction"
                      : monitor.SpendingUTXOTransaction
                      ? "Spending UTXO Transaction"
                      : "Unknown"}
                  </Link>
                </TableCell>
                <TableCell className="text-left">
                  {monitor.Transaction && (
                    <div className="flex flex-col gap-2">
                      <div>
                        <span className="font-semibold">TXID:</span>{" "}
                        <TransactionLink txId={monitor.Transaction?.[0]} />
                      </div>
                      <div>
                        <span className="font-semibold">Extra Data:</span>{" "}
                        <pre className="bg-gray-100 p-2 rounded-md overflow-x-auto">
                          {JSON.stringify(monitor.Transaction?.[1], null, 2)}
                        </pre>
                      </div>
                    </div>
                  )}
                  {monitor.SpendingUTXOTransaction && (
                    <div className="flex flex-col gap-1">
                      <div>
                        <span className="font-semibold">TXID:</span>{" "}
                        <TransactionLink
                          txId={monitor.SpendingUTXOTransaction?.[0]}
                        />
                      </div>
                      <div>
                        <span className="font-semibold">VOUT:</span>{" "}
                        {monitor.SpendingUTXOTransaction?.[1]}
                      </div>
                      <div>
                        <span className="font-semibold">Extra Data:</span>{" "}
                        {monitor.SpendingUTXOTransaction?.[2]}
                      </div>
                    </div>
                  )}

                  {monitor.RskPeginTransaction && (
                    <div className="flex flex-col gap-1">
                      <span className="font-semibold">TXID:</span>{" "}
                      <TransactionLink
                        txId={monitor.RskPeginTransaction?.[0]}
                      />
                    </div>
                  )}
                  {monitor.toString() === "NewBlock" && (
                    <div className="flex flex-col gap-1">ON</div>
                  )}
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      )}
    </div>
  );
}
