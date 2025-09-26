import { ChallengeStatusDisplay } from "../common/challenge-status-display";
import { useCurrentGame } from "@/hooks/useGame";
export function WaitingAnswer() {
  const { data: game } = useCurrentGame();

  return (
    <div className="space-y-4 border border-gray-200 rounded-md p-4">
      <ChallengeStatusDisplay
        transactions={
          game?.bitvmx_program_properties.txs as {
            [key: string]: {
              tx_id: string;
              block_info?: { height?: number; hash?: string };
            };
          }
        }
      />
    </div>
  );
}
