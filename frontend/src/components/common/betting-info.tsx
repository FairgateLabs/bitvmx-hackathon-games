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
    placeBet({ program_id: game?.program_id ?? "", amount: 1e8 });
  };

  useEffect(() => {
    setIsOpen(true);
  }, []);

  return (
    <div className="p-4 rounded-lg border border-gray-200">
      <Collapsible open={isOpen} onOpenChange={setIsOpen}>
        <CollapsibleTrigger asChild>
          <h3 className="font-semibold mb-2 cursor-pointer hover:text-gray-900">
            💰 Bet & Fund Game
          </h3>
        </CollapsibleTrigger>
        <CollapsibleContent>
          <div className="space-y-10">
            <p className="text-sm text-gray-700">
              The amount to bet for this game will be <strong>1 BTC</strong> +
              protocol fee.
              <br />
              In this game setup, Player 1 will cover both the protocol fee and
              the bet amount. Player 2 is not required to pay anything. The
              original idea was for each player to pay their share of the
              protocol fee and the bet amount. However, this approach introduces
              complexity in constructing the transaction fundings. To keep
              things simple, Player 1 will handle all payments. <br />
              When you accept this bet, a funding transaction will be
              automatically performed by the backend. This transaction will move
              the required funding amount from your provided wallet to the
              aggregated wallet that was generated earlier for this game
              session.
            </p>

            <Button
              onClick={handleAcceptBet}
              disabled={isPlacingBet}
              className="w-full bg-gray-600 hover:bg-gray-700"
            >
              {isPlacingBet
                ? "⏳ Setting Up..."
                : "🔗 Accept to bet 1 BTC + Protocol Fee"}
            </Button>
          </div>
        </CollapsibleContent>
      </Collapsible>
    </div>
  );
}
