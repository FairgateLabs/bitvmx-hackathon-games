import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

export function AnswerGame() {
  const [answer, setAnswer] = useState("");
  const [isSubmitted, setIsSubmitted] = useState(false);

  const isAnswerValid = () => {
    const parsedAnswer = parseInt(answer, 10);
    return !isNaN(parsedAnswer) && parsedAnswer >= 0;
  };

  const handleSubmit = () => {
    if (isAnswerValid()) {
      // useAnswerAddNumber(id, answer);
      setIsSubmitted(true);
    } else {
      setIsSubmitted(false);
    }
  };

  return (
    <div className="space-y-4">
      <div className="space-y-4 bg-blue-50 border border-blue-200 rounded-md p-4">
        <h3 className="text-lg font-semibold">🧮 Start Game!</h3>
        <p className="text-sm text-blue-700">
          Now that you&apos;ve set up the game, what is the sum?
        </p>

        <div>
          <Label htmlFor="answer">Your Answer</Label>
          <Input
            id="answer"
            type="number"
            value={answer}
            onChange={(e) => {
              const value = e.target.value;
              if (parseInt(value, 10) >= 0 || value === "") {
                setAnswer(value);
              }
            }}
            placeholder="Enter the sum"
            disabled={isSubmitted}
          />
        </div>

        <Button
          onClick={handleSubmit}
          disabled={!isAnswerValid() || isSubmitted}
          className="w-full"
        >
          Send Answer
        </Button>

        {!isSubmitted && (
          <div className="p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
            <h3 className="font-semibold mb-2 text-yellow-800">
              ⚠️ Provide your answer to proceed
            </h3>
            <p className="text-sm text-yellow-700">
              Please calculate the sum and enter your answer to continue.
            </p>
          </div>
        )}

        {isSubmitted && (
          <div className="p-4 bg-green-50 border border-green-200 rounded-lg">
            <h3 className="font-semibold mb-2 text-green-800">
              ✅ Answer Submitted Successfully
            </h3>
            <p className="text-sm text-green-700">
              Answer submitted successfully.
            </p>
          </div>
        )}
      </div>
    </div>
  );
}
