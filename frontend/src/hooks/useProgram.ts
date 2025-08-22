import { useQuery } from "@tanstack/react-query";

type Program = {
  program_id: string;
  my_idx: number;
  participants: Participant[];
  state: string;
  protocol: Protocol;
};

type Protocol = {
  LockProtocol: {
    ctx: ProtocolContext;
  };
  DisputeResolutionProtocol: {
    ctx: ProtocolContext;
  };
};

type ProtocolContext = {
  id: string;
  my_idx: number;
  protocol_name: string;
};

type Participant = {
  p2p_address: Address;
  keys: Keys;
  nonces: NonceEntry[];
  partial: NonceEntry[];
};

type TxSignature = [string, string];

type NonceEntry = [
  string, // pubKeyA
  string, // pubKeyB
  TxSignature[] // tx signatures
];

type Address = {
  address: string;
  peer_id: string;
};

type PublicKey = { Public: string };

type WinternitzHash = { hash: number[] };

type Winternitz = {
  hashes: WinternitzHash[];
  hash_type: string;
  extra_data: {
    message_size: number;
    checksum_size: number;
    derivation_index: number;
  };
};

type Mapping = Map<string, PublicKey | Winternitz>;

type Keys = {
  mapping: Mapping;
  aggregated: string[];
  computed_aggregated: Record<string, string>;
};

const fetchProgram = async (programId: string) => {
  try {
    const response = await fetch(`http://localhost:8080/program/${programId}`);
    const program = JSON.parse(await response.json());
    return program;
  } catch (error) {
    throw new Error("Error fetching programs: " + error);
  }
};

const useProgram = (programId: string) => {
  return useQuery<Program, Error>({
    queryKey: ["program", programId],
    queryFn: () => fetchProgram(programId),
    enabled: !!programId,
  });
};

export default useProgram;
