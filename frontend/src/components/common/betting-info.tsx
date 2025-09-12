import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { useCurrentGame } from "@/hooks/useGame";
import { usePlaceBet } from "@/hooks/usePlaceBet";

export function BettingInfo() {
  const [isOpen, setIsOpen] = useState(true);
  const { mutate: placeBet, isPending: isPlacingBet } = usePlaceBet();
  const { data: game } = useCurrentGame();

  const handleAcceptBet = () => {
    placeBet({ gameId: game?.program_id ?? "", amount: 1e8 });
  };

  useEffect(() => {
    setIsOpen(true);
  }, []);

  return (
    <div className="p-4 bg-white border border-gray-200 rounded-lg">
      <Collapsible open={isOpen} onOpenChange={setIsOpen}>
        <CollapsibleTrigger asChild>
          <h3 className="font-semibold mb-2 cursor-pointer hover:text-gray-900">
            💰 Betting Information
          </h3>
        </CollapsibleTrigger>
        <CollapsibleContent>
          <div className="text-center space-y-6">
            <div className="bg-gray-50 border border-gray-200 rounded-lg p-4 mb-4">
              <h3 className="text-lg font-semibold text-gray-800 mb-2">
                ⚠️ Important: Betting Amount
              </h3>
              <p className="text-gray-700">
                The amount to bet for this game will be <strong>1 BTC</strong>
              </p>
            </div>

            <div className="bg-gray-50 border border-gray-200 rounded-lg p-4 mb-6">
              <h3 className="text-lg font-semibold text-gray-800 mb-2">
                🔄 Funding Transaction Process
              </h3>
              <p className="text-gray-700 text-sm leading-relaxed">
                When you accept this bet, a funding transaction will be
                automatically performed by the backend. This transaction will
                move the required funding amount from your provided wallet to
                the aggregated wallet that was generated earlier for this game
                session.
              </p>
            </div>

            <div className="space-y-4">
              <div className="flex items-center justify-center space-x-2 text-sm text-gray-600">
                <span>Game ID:</span>
                <code className="bg-gray-100 px-2 py-1 rounded font-mono">
                  {game?.program_id}
                </code>
              </div>

              <div className="flex items-center justify-center space-x-2 text-sm text-gray-600">
                <span>Bet Amount:</span>
                <span className="font-semibold text-gray-800">1 BTC</span>
              </div>
            </div>

            {!isOpen ? (
              <div className="text-center">
                <Button
                  onClick={handleAcceptBet}
                  disabled={isPlacingBet}
                  className="bg-gray-600 hover:bg-gray-700 text-white px-8 py-3 text-lg font-semibold"
                >
                  {isPlacingBet
                    ? "⏳ Processing..."
                    : "✅ Accept Bet & Fund Game"}
                </Button>
                <p className="text-xs text-gray-500 mt-2">
                  By clicking accept, you agree to fund this game with 1 BTC
                </p>
              </div>
            ) : (
              <div className="text-center">
                <div className="bg-gray-50 border border-gray-200 rounded-lg p-4">
                  <h3 className="text-lg font-semibold text-gray-800 mb-2">
                    ✅ Bet Accepted Successfully!
                  </h3>
                  <p className="text-gray-700">
                    Your bet has been placed and the funding transaction is
                    being processed. The game can now proceed.
                  </p>
                </div>
              </div>
            )}
          </div>
        </CollapsibleContent>
      </Collapsible>
    </div>
  );
}
