import { useQuery } from "@tanstack/react-query";

const fetchCurrentBlockHeight = async () => {
  try {
    const response = await fetch(
      "http://localhost:8080/monitor/height/current"
    );
    const blockHeight = await response.json();
    return blockHeight;
  } catch (error) {
    throw new Error("Error fetching block height: " + error);
  }
};

const useMonitorHeight = () => {
  return useQuery({
    queryKey: ["monitorHeight"],
    queryFn: fetchCurrentBlockHeight,
    refetchInterval: 10000, // 10 seconds
    refetchOnWindowFocus: true,
  });
};

export default useMonitorHeight;
