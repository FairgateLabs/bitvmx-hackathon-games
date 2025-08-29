import Link from "next/link";

export function TransactionLink({ txId }: { txId: string }) {
  return (
    <Link
      href={`/monitor/search-tx?id=${txId}`}
      className="font-mono text-sm truncate hover:underline cursor-pointer"
    >
      {truncateMiddle(txId)}
    </Link>
  );
}

export function truncateMiddle(str: string, startLen = 8, endLen = 8): string {
  if (str.length <= startLen + endLen + 3) {
    return str;
  }
  const start = str.slice(0, startLen);
  const end = str.slice(-endLen);
  return `${start}...${end}`;
}
