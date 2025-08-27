import { useQuery } from "@tanstack/react-query";

interface AddressInfo {
  address: string;
}

const fetchAddressInfo = async (): Promise<AddressInfo> => {
  // const response = await fetch("/api/address-info", {
  //   method: "GET",
  //   headers: {
  //     "Content-Type": "application/json",
  //   },
  // });

  // if (!response.ok) {
  //   throw new Error("Failed to fetch address info");
  // }
  return { address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa" };

  // return response.json();
};

// Hook for getting player address
export const useAddress = () => {
  return useQuery({
    queryKey: ["address"],
    queryFn: fetchAddressInfo,
  });
};
