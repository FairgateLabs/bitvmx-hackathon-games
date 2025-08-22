import { useQuery } from "@tanstack/react-query";

const fetchTransaction = async (txid: string) => {
  try {
    const response = await fetch(
      `http://localhost:8080/monitor/transaction/${txid}`
    );
    const monitor = await response.json();
    return monitor;
  } catch (error) {
    throw new Error("Error fetching transaction: " + error);
  }
};

type TransactionInfo = {
  tx: Object;
  block_height: number;
  block_hash: string;
  orphan: boolean;
  confirmations: number;
};

const useTransaction = (txid: string) => {
  return useQuery<TransactionInfo, Error>({
    queryKey: ["transaction", txid],
    queryFn: () => fetchTransaction(txid),
    enabled: !!txid,
  });
};

export default useTransaction;
