import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useAnswerAddNumber, useCurrentGame } from "@/hooks/useGame";

export function SubmitGameData() {
  const [guess, setGuess] = useState<number | undefined>(undefined);
  const { data: game } = useCurrentGame();
  const { mutate: submitSum, isPending, isSuccess } = useAnswerAddNumber();

  const isAnswerValid = () => {
    const parsedAnswer = parseInt(guess?.toString() ?? "0", 10);
    return !isNaN(parsedAnswer) && parsedAnswer >= 0;
  };

  return (
    <div className="space-y-4">
      <div className="space-y-4  border border-gray-200 rounded-md p-4">
        <h3 className="text-lg font-semibold">➕ Submit Answer Sum!</h3>
        <p className="text-sm text-gray-700">
          Now that you&apos;ve set up the game, what is the sum?
        </p>

        <div>
          <Label htmlFor="answer">Your Answer</Label>
          <Input
            id="answer"
            type="number"
            onChange={(e) => {
              const value = e.target.value;
              if (parseInt(value, 10) >= 0 || value === "") {
                const guess = parseInt(value, 10);
                setGuess(guess);
              }
            }}
            placeholder="Enter the sum"
            disabled={isPending}
          />
        </div>

        <Button
          onClick={() =>
            submitSum({
              id: game?.program_id ?? "",
              guess: guess ?? 0,
            })
          }
          disabled={isPending || !isAnswerValid()}
          className="w-full"
        >
          <span className="flex items-center justify-center">
            {isPending ? "⏳ Submitting Answer Sum..." : "➕ Submit Answer Sum"}
          </span>
        </Button>

        {!isPending && (
          <div className="p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
            <h3 className="font-semibold mb-2 text-yellow-800">
              ⚠️ Provide your answer to proceed
            </h3>
            <p className="text-sm text-yellow-700">
              Please input the sum and click the button to submit your answer.
            </p>
          </div>
        )}

        <div className="p-4 bg-yellow-50 border border-yellow-200 rounded-lg mt-4">
          <h3 className="font-semibold mb-2 text-yellow-800">
            ⚠️ On-Chain Verification
          </h3>
          <p className="text-sm text-yellow-700">
            This process will take some time. Player 1 has initiated an on-chain
            request to prove the sum of the game. You will submit your answer
            on-chain and dispute the challenge there to determine the winner.
          </p>
        </div>
      </div>
    </div>
  );
}
