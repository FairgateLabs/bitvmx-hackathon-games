import { useQuery } from "@tanstack/react-query";
import { getApiBaseUrl } from "../config/backend";

interface AddressInfo {
  address: string;
}

const fetchAddressInfo = async (): Promise<AddressInfo> => {
  // const baseUrl = getApiBaseUrl();
  // const response = await fetch(`${baseUrl}/api/address-info`, {
  //   method: "GET",
  //   headers: {
  //     "Content-Type": "application/json",
  //   },
  // });

  // if (!response.ok) {
  //   throw new Error("Failed to fetch address info");
  // }
  // return response.json();

  return { address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa" };
};

// Hook for getting player address
export const useAddress = () => {
  return useQuery({
    queryKey: ["address"],
    queryFn: fetchAddressInfo,
  });
};
