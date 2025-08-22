import { useQuery } from "@tanstack/react-query";

const fetchBlocks = async () => {
  try {
    const response = await fetch("http://localhost:8080/indexer/last_blocks");
    const blocks = await response.json();
    return blocks;
  } catch (error) {
    throw new Error("Error fetching blocks: " + error);
  }
};

type Block = {
  hash: string;
  prev_hash: string;
  height: number;
  orphan: boolean;
  txs: { 0: string; 1: Object }[];
};

const useBlocks = () => {
  return useQuery<Block[], Error>({
    queryKey: ["blocks"],
    queryFn: fetchBlocks,
    refetchInterval: 10000, // 10 seconds
    refetchOnWindowFocus: true,
  });
};

export default useBlocks;
