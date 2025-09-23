import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { useCurrentGame } from "@/hooks/useGame";
import { usePlaceBet } from "@/hooks/usePlaceBet";
import { useQueryClient } from "@tanstack/react-query";

export function PlaceBet() {
  const [isOpen, setIsOpen] = useState(true);
  const { mutate: placeBet, isPending: isPlacingBet } = usePlaceBet();
  const { data: game } = useCurrentGame();
  const queryClient = useQueryClient();

  const handleAcceptBet = () => {
    placeBet({ program_id: game?.program_id ?? "", amount: 1e4 });
  };

  useEffect(() => {
    setIsOpen(true);
  }, []);

  useEffect(() => {
    queryClient.invalidateQueries({ queryKey: ["currentGameId"] });
  }, [isPlacingBet]);

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
              {game?.role === "Player1" ? (
                <>
                  The amount to bet for this game will be{" "}
                  <strong>0.0001 BTC</strong> + protocol fee.
                  <br />
                  In this game setup, you will cover both the protocol fee and
                  the bet amount. Player 2 is not required to pay anything.
                  <br />
                  When you accept this bet, a funding transaction will be
                  automatically performed by the backend. This transaction will
                  move the required funding amount from your provided wallet to
                  the aggregated wallet that was generated earlier for this game
                  session.
                </>
              ) : (
                <>
                  As Player 2, you are not required to pay any amount for the
                  protocol or the bet. Player 1 will handle all the necessary
                  payments for this game session.
                  <br />
                  Please wait for Player 1 to complete the funding transaction.
                </>
              )}
            </p>

            <Button
              onClick={handleAcceptBet}
              disabled={isPlacingBet}
              className="w-full bg-gray-600 hover:bg-gray-700"
            >
              {isPlacingBet
                ? "⏳ Setting Up..."
                : "🔗 Accept to bet 0.0001 BTC + Protocol Fee"}
            </Button>
          </div>
        </CollapsibleContent>
      </Collapsible>
    </div>
  );
}
