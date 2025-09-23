import { useQuery } from "@tanstack/react-query";
import { instance } from "@viz-js/viz";

const fetchProtocol = async (protocolId: string) => {
  try {
    const response = await fetch(
      `http://localhost:8080/protocol/${protocolId}`
    );
    const protocol = await response.json();
    const inst = await instance();
    const svg = inst.renderSVGElement(protocol);
    return svg;
  } catch (error) {
    throw new Error("Error fetching programs: " + error);
  }
};

const useProtocol = (protocolId: string) => {
  return useQuery({
    queryKey: ["protocol", protocolId],
    queryFn: () => fetchProtocol(protocolId),
    enabled: !!protocolId,
  });
};

export default useProtocol;
