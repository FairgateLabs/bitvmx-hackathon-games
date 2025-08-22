import { useQuery } from "@tanstack/react-query";

const fetchMonitors = async () => {
  try {
    const response = await fetch("http://localhost:8080/monitor/all");
    const monitors = await response.json();
    return monitors;
  } catch (error) {
    throw new Error("Error fetching monitors: " + error);
  }
};

const useMonitors = () => {
  return useQuery({
    queryKey: ["monitors"],
    queryFn: fetchMonitors,
    refetchInterval: 10000, // 10 seconds
    refetchOnWindowFocus: true,
  });
};

export default useMonitors;
