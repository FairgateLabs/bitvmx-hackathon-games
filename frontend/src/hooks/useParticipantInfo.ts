import { useMutation } from "@tanstack/react-query";
import { getApiBaseUrl } from "../config/backend";
import { P2PAddress } from "../../../backend/bindings/P2PAddress";
import { SetupParticipantsRequest } from "../../../backend/bindings/SetupParticipantsRequest";
import { PlayerRole } from "../../../backend/bindings/PlayerRole";

const saveParticipantInfo = async ({
  role,
  participants_addresses,
  operator_keys,
  aggregated_id,
}: {
  role: PlayerRole;
  participants_addresses: P2PAddress[];
  operator_keys: string[];
  aggregated_id: string;
}) => {
  const data: SetupParticipantsRequest = {
    role,
    aggregated_id,
    participants_addresses: participants_addresses,
    participants_keys: operator_keys,
    leader_idx: 0,
  };

  const baseUrl = getApiBaseUrl();
  const response = await fetch(
    `${baseUrl}/api/add-numbers/setup-participants`,
    {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    }
  );

  if (!response.ok) {
    throw new Error("Failed to save peer connection info");
  }
};

const useSaveParticipantInfo = () => {
  return useMutation({
    mutationFn: saveParticipantInfo,
  });
};

export { useSaveParticipantInfo };
