import { ClockIcon } from "lucide-react";
import { useState, useEffect, useImperativeHandle, forwardRef } from "react";

interface TimeRemainingProps {
  numberBlocks: number; // number of blocks
  onTimeout: () => void;
  size?: "sm" | "md" | "lg" | "xl";
}

export interface TimeRemainingRef {
  reset: () => void;
}

export const TimeRemaining = forwardRef<TimeRemainingRef, TimeRemainingProps>(
  ({ numberBlocks, onTimeout, size = "sm" }, ref) => {
    const timePerBlock = 2; // Each block is estimated to be 30 seconds in regtest
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
    }, [initialTime, numberBlocks]);

    useEffect(() => {
      if (timeLeft > 0) {
        const timer = setInterval(() => {
          setTimeLeft((prev) => {
            const newTime = prev - 1;
            if (newTime <= 0) {
              onTimeout();
              return 0;
            }
            return newTime;
          });

          setBlocksLeft((prev) => {
            const timeElapsed = initialTime - timeLeft;
            const newBlocks = Math.max(
              0,
              numberBlocks - Math.floor(timeElapsed / timePerBlock)
            );
            return newBlocks;
          });
        }, 1000);
        return () => clearInterval(timer);
      } else {
        setBlocksLeft(0); // Ensure blocks count is 0 when time is 0
      }
    }, [timeLeft, numberBlocks]);

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
);
