"use client";

import { useState, useEffect, useRef } from "react";
import { Button } from "../../ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "../../ui/card";
import { EnumPlayerRole } from "@/types/game";
import { useCurrentGame } from "@/hooks/useGame";
import { useMockTicTacToeMoves } from "@/hooks/useTicTacToeMoves";
import { TimeRemaining, TimeRemainingRef } from "../../ui/time-remaining"; // Import the timer component
import { PlayerRole } from "../../../../../backend/bindings/PlayerRole";

export enum PlayerSymbol {
  X = "X",
  O = "O",
}

export type Board = PlayerSymbol[];

export interface GameEndResult {
  winner: EnumPlayerRole | null; // null means draw
  isTimeout: boolean;
}

export interface MoveLog {
  moveNumber: number;
  player: EnumPlayerRole;
  position: number;
  symbol: PlayerSymbol;
  timestamp: Date;
  replacedMove?: number; // If this move replaced a previous move
}

interface TicTacToeBoardProps {
  onGameEnd: (result: GameEndResult) => void;
}

export function TicTacToeBoard({ onGameEnd }: TicTacToeBoardProps) {
  const [board, setBoard] = useState<Board>(Array(9).fill(null));
  const [winner, setWinner] = useState<PlayerRole | null>(null);
  const [gameOver, setGameOver] = useState(false);
  const [disabled, setDisabled] = useState(false);
  const [currentPlayer, setCurrentPlayer] = useState<EnumPlayerRole>(
    EnumPlayerRole.Player1
  );
  const [moveCount, setMoveCount] = useState(0);
  const [movesLog, setMovesLog] = useState<MoveLog[]>([]);
  const { data: currentGame } = useCurrentGame();
  const playerRole = currentGame?.role;
  const timerRef = useRef<TimeRemainingRef>(null);

  const { data: opponentMove, isLoading: isLoadingMoves } =
    useMockTicTacToeMoves();

  // Check for winner after each move
  useEffect(() => {
    const gameWinner = checkWinner(board);

    if (gameWinner) {
      setWinner(gameWinner);
      setGameOver(true);
      onGameEnd({ winner: gameWinner, isTimeout: false });
    } else if (moveCount >= 9) {
      // Game ends after 9 moves (max possible moves)
      setGameOver(true);
      onGameEnd({ winner: null, isTimeout: false }); // null means draw
    }
  }, [board]);

  useEffect(() => {
    if (!opponentMove || gameOver || !isOpponentTurn()) return;

    if (board[opponentMove.index] === opponentMove.playerSymbol) return;

    const newBoard = [...board];
    const wasEmpty = !board[opponentMove.index];

    newBoard[opponentMove.index] = opponentMove.playerSymbol;
    setBoard(newBoard);
    setCurrentPlayer(playerRole as EnumPlayerRole);

    setMoveCount((prev) => Math.min(prev + 1, 9));

    // Add move to log with the updated moveCount
    const newMove: MoveLog = {
      moveNumber: moveCount + 1,
      player:
        opponentMove.player === EnumPlayerRole.Player1
          ? EnumPlayerRole.Player1
          : EnumPlayerRole.Player2,
      position: opponentMove.index,
      symbol: opponentMove.playerSymbol,
      timestamp: new Date(),
      replacedMove: wasEmpty ? undefined : moveCount, // Use previous moveCount for replacement
    };

    setMovesLog((prevLog) => [...prevLog, newMove]);

    timerRef.current?.reset();
    setDisabled(false);
  }, [opponentMove, gameOver, board, playerRole]);

  const checkWinner = (currentBoard: Board): PlayerRole | null => {
    const winningCombinations = [
      [0, 1, 2], // Top row
      [3, 4, 5], // Middle row
      [6, 7, 8], // Bottom row
      [0, 3, 6], // Left column
      [1, 4, 7], // Middle column
      [2, 5, 8], // Right column
      [0, 4, 8], // Diagonal
      [2, 4, 6], // Anti-diagonal
    ];

    for (const [a, b, c] of winningCombinations) {
      if (
        currentBoard[a] &&
        currentBoard[a] === currentBoard[b] &&
        currentBoard[a] === currentBoard[c]
      ) {
        return currentBoard[a] === PlayerSymbol.X
          ? EnumPlayerRole.Player1
          : EnumPlayerRole.Player2;
      }
    }

    return null;
  };

  // Determine if it's the current player's turn
  const isMyTurn = () => {
    return playerRole === currentPlayer;
  };

  const isOpponentTurn = () => {
    return playerRole !== currentPlayer;
  };

  const handleCellClick = (index: number) => {
    if (gameOver || disabled || !isMyTurn() || moveCount >= 9) return;

    const newBoard = [...board];
    const wasEmpty = !board[index];

    let myCurrentSymbol =
      playerRole === EnumPlayerRole.Player1 ? PlayerSymbol.X : PlayerSymbol.O;

    if (board[index] === myCurrentSymbol) return;

    newBoard[index] =
      playerRole === EnumPlayerRole.Player1 ? PlayerSymbol.X : PlayerSymbol.O;
    setBoard(newBoard);
    setMoveCount((prev) => Math.min(prev + 1, 9));

    // Add move to log
    const newMove: MoveLog = {
      moveNumber: moveCount + 1,
      player: playerRole as EnumPlayerRole,
      position: index,
      symbol: myCurrentSymbol,
      timestamp: new Date(),
      replacedMove: wasEmpty ? undefined : moveCount, // If position wasn't empty, this replaced the previous move
    };

    setMovesLog((prev) => [...prev, newMove]);

    setCurrentPlayer(
      playerRole === EnumPlayerRole.Player1
        ? EnumPlayerRole.Player2
        : EnumPlayerRole.Player1
    );

    timerRef.current?.reset();
    setDisabled(true);
  };

  const getCellColor = (cell: PlayerSymbol) => {
    if (cell === PlayerSymbol.X)
      return "text-blue-600 bg-blue-50 border-blue-200";
    if (cell === PlayerSymbol.O) return "text-red-600 bg-red-50 border-red-200";
    return "text-gray-600 bg-white border-gray-200 hover:bg-gray-50";
  };

  return (
    <Card className="w-full max-w-4xl mx-auto">
      <div className="flex gap-4 p-4">
        <div className="flex-1">
          <div className="text-center mb-4">
            <h3 className={"text-sm font-medium"}>
              Current turn:{" "}
              {playerRole == currentPlayer ? "Your turn!" : "Opponent's turn"}
            </h3>
          </div>

          <div
            className={`flex justify-center ${disabled ? "opacity-50" : ""}`}
          >
            <div className="grid grid-cols-3 w-[235px] h-[235px] gap-0 bg-gray-50 rounded-lg">
              {board.map((cell, index) => (
                <Button
                  key={index}
                  variant="outline"
                  className={`h-20 w-20 text-2xl font-bold border-2 rounded-none ${getCellColor(
                    cell
                  )} ${
                    disabled || gameOver
                      ? "cursor-not-allowed"
                      : "cursor-pointer"
                  }`}
                  onClick={() => handleCellClick(index)}
                  disabled={disabled || gameOver}
                >
                  {cell}
                </Button>
              ))}
            </div>
          </div>
        </div>

        {/* Moves Log - Right Side */}
        <div className="flex-1">
          <div className="text-center mb-4">
            <h3 className="text-sm font-medium text-gray-700">
              📝 Moves Log (Move {moveCount + 1}/9)
            </h3>
          </div>

          <div className="max-h-80 overflow-y-auto space-y-2">
            {movesLog.length === 0 ? (
              <p className="text-sm text-gray-500 text-center py-4">
                No moves yet...
              </p>
            ) : (
              movesLog.map((move, index) => (
                <div
                  key={index}
                  className={`text-xs p-2 rounded border-l-4 ${
                    move.replacedMove
                      ? "bg-orange-50 border-orange-300 text-orange-800"
                      : move.player === EnumPlayerRole.Player1
                      ? "bg-blue-50 border-blue-300 text-blue-800"
                      : "bg-red-50 border-red-300 text-red-800"
                  }`}
                >
                  <div className="font-medium">
                    {move.player === EnumPlayerRole.Player1
                      ? "🔵 Player 1"
                      : "🔴 Player 2"}{" "}
                    ({move.symbol}) at position [{move.position}]
                    {move.replacedMove && (
                      <span className="text-orange-600 font-medium ml-2">
                        ⚠️ Replaced move {move.replacedMove}
                      </span>
                    )}
                  </div>
                </div>
              ))
            )}
          </div>
        </div>
      </div>

      {/* Timer at the bottom */}
      {!gameOver && (
        <div className="px-4 pb-4">
          <div className="flex justify-center">
            <TimeRemaining
              ref={timerRef}
              numberBlocks={5} // 30 seconds timeout
              onTimeout={() => {
                const timeoutWinner =
                  currentPlayer === EnumPlayerRole.Player1
                    ? EnumPlayerRole.Player2
                    : EnumPlayerRole.Player1;

                setGameOver(true);
                setWinner(timeoutWinner);
                onGameEnd({ winner: timeoutWinner, isTimeout: true });
              }}
              size="sm"
            />
          </div>
        </div>
      )}
    </Card>
  );
}
