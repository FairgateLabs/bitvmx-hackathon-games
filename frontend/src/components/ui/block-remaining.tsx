import { ClockIcon } from "lucide-react";
import { useState, useEffect, useImperativeHandle, forwardRef } from "react";

interface BlocksRemainingProps {
  numberBlocks: number; // number of blocks
  onTimeout: () => void;
}

export interface BlocksRemainingRef {
  reset: () => void;
}

export const BlocksRemaining = forwardRef<
  BlocksRemainingRef,
  BlocksRemainingProps
>(function TimeRemaining({ numberBlocks, onTimeout }, ref) {
  const timePerBlock = 30; // Each block is estimated to be 30 seconds in regtest
  const initialTime = numberBlocks * timePerBlock;
  const [timeLeft, setTimeLeft] = useState(initialTime);
  const [blocksLeft, setBlocksLeft] = useState(numberBlocks);

  // Expose methods to parent component
  useImperativeHandle(ref, () => ({
    reset: () => {
      setTimeLeft(initialTime);
      setBlocksLeft(numberBlocks);
    },
  }));

  useEffect(() => {
    setTimeLeft(initialTime);
    setBlocksLeft(numberBlocks);
  }, [initialTime, numberBlocks, onTimeout]);

  useEffect(() => {
    if (timeLeft > 0) {
      const timer = setInterval(() => {
        setTimeLeft((prevTime) => {
          const newTime = prevTime - 1;
          if (newTime <= 0) {
            onTimeout();
            return 0;
          }
          return newTime;
        });

        const timeElapsed = initialTime - timeLeft;
        const newBlocks = Math.max(
          0,
          numberBlocks - Math.floor(timeElapsed / timePerBlock)
        );

        setBlocksLeft(newBlocks);
      }, 1000);
      return () => clearInterval(timer);
    } else {
      setBlocksLeft(0); // Ensure blocks count is 0 when time is 0
    }
  }, [timeLeft, numberBlocks]);

  return (
    <div
      className={`sm text-gray-800 font-semibold items-center gap-4 flex justify-center py-3`}
    >
      <div className="flex items-center gap-2">
        <span className="block text-sm text-gray-500">Blocks remaining</span>
        <span className="text-lg"> {blocksLeft}</span>
      </div>
    </div>
  );
});
