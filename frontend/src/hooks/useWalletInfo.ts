import { useQuery } from "@tanstack/react-query";
import { getApiBaseUrl } from "../config/backend";
import { OperatorKeys } from "../../../backend/bindings/OperatorKeys";

//TODO: use backend binding when is available
export type WalletBalance = {
  address: string;
  balance: number;
};

const fetchWalletBalance = async (): Promise<WalletBalance> => {
  const baseUrl = getApiBaseUrl();
  const response = await fetch(`${baseUrl}/api/bitvmx/wallet-balance`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  });

  if (!response.ok) {
    throw new Error("Failed to fetch wallet balance");
  }
  let data = await response.json();
  data.balance = data.balance / 1e8;

  return {
    address: data.address,
    balance: data.balance,
  };
};

// Hook for getting player address
export const useWalletBalance = () => {
  return useQuery({
    queryKey: ["walletBalance"],
    queryFn: fetchWalletBalance,
  });
};
