import { ClockIcon } from "lucide-react";
import { useState, useEffect } from "react";

interface TimeRemainingProps {
  numberBlocks: number; // number of blocks
  onTimeout: () => void;
  size?: "sm" | "md" | "lg" | "xl";
}

export function TimeRemaining({
  numberBlocks,
  onTimeout,
  size = "sm",
}: TimeRemainingProps) {
  const timePerBlock = 3; // Each block is estimated to be 30 seconds in regtest
  const initialTime = numberBlocks * timePerBlock;
  const [timeLeft, setTimeLeft] = useState(initialTime);
  const [blocksLeft, setBlocksLeft] = useState(numberBlocks);

  useEffect(() => {
    if (timeLeft > 0) {
      const timer = setInterval(() => {
        setTimeLeft((prev) => prev - 1);
        if (
          (initialTime - timeLeft + 1) % timePerBlock === 0 &&
          blocksLeft > 0
        ) {
          setBlocksLeft((prev) => prev - 1);
        }
      }, 1000);
      return () => clearInterval(timer);
    } else {
      onTimeout();
    }
  }, [timeLeft, blocksLeft, initialTime]);

  const getSizeClasses = () => {
    switch (size) {
      case "sm":
        return "text-sm";
      case "md":
        return "text-base";
      case "lg":
        return "text-lg";
      case "xl":
        return "text-xl";
      default:
        return "text-base";
    }
  };

  return (
    <div
      className={`${getSizeClasses()} text-gray-800 font-semibold items-center gap-4 flex justify-center py-3`}
    >
      <div className="flex items-center gap-2">
        <span className="block text-sm text-gray-500">Blocks remaining</span>
        <span className="text-lg"> {blocksLeft}</span>
      </div>
      <div className="flex items-center gap-2">
        <span className="text-sm text-gray-500 flex items-center gap-1">
          <ClockIcon className="h-4 w-4" />
          Time remaining
        </span>
        <span className="text-lg">{Math.ceil(timeLeft)}s</span>
      </div>
    </div>
  );
}
