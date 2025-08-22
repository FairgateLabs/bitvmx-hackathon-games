"use client";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import {
  TransactionLink,
  truncateMiddle,
} from "@/components/ui/transaction-link";
import useMonitorHeight from "@/hooks/useMonitorBlockHeight";
import useBlocks from "@/hooks/useBlocks";

export default function Indexer() {
  const { data: blockHeight, isLoading: isBlockHeightLoading } =
    useMonitorHeight();
  const { data: blocks, isLoading: isBlocksLoading } = useBlocks();

  return (
    <div className="flex  flex-col min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
      <h1 className="text-4xl font-bold">Block Explorer</h1>

      <div className="flexjustify-end">
        <div className="flex items-center gap-2">
          <span className="text-gray-500">Indexer height:</span>
          <span className="font-mono font-bold">
            {isBlockHeightLoading && "Loading..."}
            {!isBlockHeightLoading && blockHeight}
          </span>
        </div>
      </div>
      <div className="flex">
        {isBlocksLoading && "Loading..."}
        {!isBlocksLoading && (
          <Table className="">
            <TableHeader>
              <TableRow>
                <TableHead>Block Height</TableHead>
                <TableHead>BlockHash</TableHead>
                <TableHead>Is Orphan</TableHead>
                <TableHead># Transaction</TableHead>
                <TableHead>Transactions</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {blocks?.map((block) => (
                <TableRow key={block.hash}>
                  <TableCell>{block.height}</TableCell>
                  <TableCell className="font-mono">
                    {truncateMiddle(block.hash)}
                  </TableCell>
                  <TableCell>{block.orphan ? "Yes" : "No"}</TableCell>
                  <TableCell className="text-center">
                    {block.txs.length}
                  </TableCell>
                  <TableCell>
                    <div className="flex flex-col gap-2">
                      {block.txs.slice(0, 4).map((tx) => (
                        <div key={tx[0]}>
                          <TransactionLink txId={tx[0]} />
                        </div>
                      ))}
                      {block.txs.length > 4 && (
                        <span className="text-gray-500 text-sm">
                          +{block.txs.length - 4} more
                        </span>
                      )}
                    </div>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        )}
      </div>
    </div>
  );
}
