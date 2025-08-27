import { useQuery } from "@tanstack/react-query";

const fetchPeerConnectionInfo = async () => {
  // const response = await fetch("/api/peer-connection");
  // if (!response.ok) {
  //   throw new Error("Failed to fetch peer connection info");
  // }
  // return response.json();

  return {
    address: "/ip4/127.0.0.1/tcp/61180",
    peerId:
      "30820122300d06092a864886f70d01010105000382010f003082010a0282010100b0595a239c455f955ac2617061fadc0f3c532056da4a4ab4111b6581a62143e6c00b3041a00c290232fa65794ea0a55ca5f2ed3310ecbcab06a721d66e99a27e0d1b8a6afd8e395b741fbcf6cb73294eaeff43118f828f0118a4b5fdc95d472bcadaf2bc4d665e535ccd70b8ee5b82624794351a82c9f819d9a53638122228d1800d7d6561ae98183ae53c6cf23964c7eceeae95807db49a164cfbbc1ddc87a975fbe3d43545e8ce1bad2043cfe6a9aa3a7538ebdab8e6b900c94a691c1321d7c2d7f1a1beb3c3ef03686f7805ce938c92c8d5057cb5101cd51c1d97d7d3d4b9f13b7cb28bc5c4c5c9983a3062efc606b9c440021e1d5257d88d9c3ced0ac38f0203010001",
  };
};

export const usePeerConnection = () => {
  // Fetch IP and Port from backend using react-query
  return useQuery({
    queryKey: ["peerConnectionInfo"],
    queryFn: fetchPeerConnectionInfo,
  });
};
