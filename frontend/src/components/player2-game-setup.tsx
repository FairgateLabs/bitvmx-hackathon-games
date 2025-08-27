import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

export function Player2GameSetup() {
  const [answer, setAnswer] = useState("");

  return (
    <div className="space-y-4">
      <h3 className="text-lg font-semibold">🧮 Answer Sum</h3>

      <div className="space-y-4">
        <div className="p-4 bg-blue-50 border border-blue-200 rounded-md">
          <h4 className="font-semibold text-blue-800">🎯 Calculate the Sum</h4>
          <p className="text-sm text-blue-700">
            Player 1 has chosen two numbers. What is the sum?
          </p>
        </div>

        <div>
          <Label htmlFor="answer">Your Answer</Label>
          <Input
            id="answer"
            type="number"
            value={answer}
            onChange={(e) => setAnswer(e.target.value)}
            placeholder="Enter the sum"
          />
        </div>

        <Button
          onClick={() => {}}
          disabled={!answer}
          className="w-full cursor-pointer"
        >
          Send Answer
        </Button>
      </div>
    </div>
  );
}
