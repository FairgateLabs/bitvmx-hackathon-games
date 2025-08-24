import { Button } from "@/components/ui/button";

interface GameActionsProps {
  onAccept: () => void;
  onChallenge: () => void;
  isLoading: boolean;
}

export function GameActions({
  onAccept,
  onChallenge,
  isLoading,
}: GameActionsProps) {
  return (
    <div className="space-y-4">
      <h3 className="text-lg font-semibold">🎯 Game Actions</h3>
      <p className="text-sm text-muted-foreground">
        Player 2 has sent their answer. What do you want to do?
      </p>

      <div className="grid grid-cols-2 gap-4">
        <Button
          onClick={onAccept}
          disabled={isLoading}
          className="bg-green-600 hover:bg-green-700 cursor-pointer"
        >
          ✅ Correct - Transfer Funds
        </Button>

        <Button
          onClick={onChallenge}
          disabled={isLoading}
          variant="destructive"
          className="cursor-pointer"
        >
          ⚖️ Challenge - Initiate Dispute
        </Button>
      </div>

      <p className="text-xs text-muted-foreground text-center">
        ⏰ If you do nothing, Player 2 will automatically win by timeout
      </p>
    </div>
  );
}
