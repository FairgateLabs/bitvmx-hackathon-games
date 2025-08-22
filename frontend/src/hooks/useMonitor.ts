import { MonitorType } from "@/app/helper";
import { useQuery } from "@tanstack/react-query";

const fetchMonitor = async (monitorType: MonitorType, txid: string) => {
  try {
    const response = await fetch(
      `http://localhost:8080/monitor/search?monitor_type=${monitorType}&txid=${txid}`
    );
    const monitor = await response.json();
    return monitor;
  } catch (error) {
    throw new Error("Error fetching monitor: " + error);
  }
};

const useMonitor = (monitorType: MonitorType, txid: string) => {
  return useQuery({
    queryKey: ["monitors", monitorType, txid],
    queryFn: () => fetchMonitor(monitorType, txid),
    enabled: !!monitorType && !!txid,
  });
};

export default useMonitor;
