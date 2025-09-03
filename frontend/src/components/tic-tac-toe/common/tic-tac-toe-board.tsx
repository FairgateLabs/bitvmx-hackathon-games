"use client";

import { useState, useEffect, useRef } from "react";
import { Button } from "../../ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "../../ui/card";
import { Player, PlayerRole } from "@/types/game";
import { useGameRole } from "@/hooks/useGameRole";
import { useMockTicTacToeMoves } from "@/hooks/useTicTacToeMoves";
import { TimeRemaining, TimeRemainingRef } from "../../ui/time-remaining"; // Import the timer component
import { useGame } from "@/hooks/useGame";

export enum PlayerSymbol {
  X = "X",
  O = "O",
}

export type Board = PlayerSymbol[];

interface TicTacToeBoardProps {
  onGameEnd: (winner: Player, board: Board) => void;
}

export function TicTacToeBoard({ onGameEnd }: TicTacToeBoardProps) {
  const [board, setBoard] = useState<Board>(Array(9).fill(null));
  const [winner, setWinner] = useState<Player | null>(null);
  const [gameOver, setGameOver] = useState(false);
  const [disabled, setDisabled] = useState(false);
  const [currentPlayer, setCurrentPlayer] = useState<PlayerRole>(
    PlayerRole.Player1
  );
  const [timeoutOccurred, setTimeoutOccurred] = useState(false);
  const { data: playerRole } = useGameRole();
  const timerRef = useRef<TimeRemainingRef>(null);
  const { data: gameId } = useGame();

  const { data: opponentMove, isLoading: isLoadingMoves } =
    useMockTicTacToeMoves(gameId);

  // Handle timeout
  useEffect(() => {
    if (timeoutOccurred) {
      const timeoutWinner =
        currentPlayer === PlayerRole.Player1
          ? PlayerRole.Player2
          : PlayerRole.Player1;

      setGameOver(true);
      setWinner(timeoutWinner);
      onGameEnd(timeoutWinner, board);
      timerRef.current?.reset();
    }
  }, [timeoutOccurred, currentPlayer, board, onGameEnd]);

  // Check for winner after each move
  useEffect(() => {
    const gameWinner = checkWinner(board);

    if (gameWinner) {
      setWinner(gameWinner);
      setGameOver(true);
      onGameEnd(gameWinner, board);
    } else if (board.every((cell) => cell !== null)) {
      setGameOver(true);
    }
  }, [board, onGameEnd]);

  useEffect(() => {
    if (!opponentMove || gameOver || !isOpponentTurn()) return;

    if (board[opponentMove.index] === opponentMove.playerSymbol) return;

    const newBoard = [...board];
    newBoard[opponentMove.index] = opponentMove.playerSymbol;
    setBoard(newBoard);
    setCurrentPlayer(playerRole as PlayerRole);
    timerRef.current?.reset();
    setTimeoutOccurred(false); // Reset timeout flag
    setDisabled(false);
  }, [opponentMove, gameOver, board, playerRole]);

  const checkWinner = (currentBoard: Board): Player | null => {
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
          ? PlayerRole.Player1
          : PlayerRole.Player2;
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
    if (gameOver || disabled || !isMyTurn()) return;

    const newBoard = [...board];

    let myCurrentSymbol =
      playerRole === PlayerRole.Player1 ? PlayerSymbol.X : PlayerSymbol.O;

    if (board[index] === myCurrentSymbol) return;

    newBoard[index] =
      playerRole === PlayerRole.Player1 ? PlayerSymbol.X : PlayerSymbol.O;
    setBoard(newBoard);

    setCurrentPlayer(
      playerRole === PlayerRole.Player1
        ? PlayerRole.Player2
        : PlayerRole.Player1
    );

    timerRef.current?.reset();
    setTimeoutOccurred(false); // Reset timeout flag
    setDisabled(true);
  };

  const getCellColor = (cell: PlayerSymbol) => {
    if (cell === PlayerSymbol.X)
      return "text-blue-600 bg-blue-50 border-blue-200";
    if (cell === PlayerSymbol.O) return "text-red-600 bg-red-50 border-red-200";
    return "text-gray-600 bg-white border-gray-200 hover:bg-gray-50";
  };

  const getStatusMessage = () => {
    if (gameOver) {
      if (winner) {
        return `🎉 Player ${winner} wins!`;
      }

      return "🤝 It's a draw!";
    }
    return `Current turn: ${
      playerRole == currentPlayer ? "Your turn!" : "Opponent's turn"
    }`;
  };

  const getStatusColor = () => {
    if (gameOver) {
      if (winner) {
        return winner === PlayerRole.Player1 ? "text-blue-600" : "text-red-600";
      }
      return "text-gray-600";
    }
    return "text-gray-700";
  };

  return (
    <Card className="w-full max-w-md mx-auto">
      <CardHeader className="pb-4">
        <CardTitle
          className={`text-center text-sm font-medium ${getStatusColor()}`}
        >
          {getStatusMessage()}
        </CardTitle>

        {!gameOver && (
          <TimeRemaining
            ref={timerRef}
            numberBlocks={3} // 30 seconds timeout
            onTimeout={() => {
              setTimeoutOccurred(true);
            }}
            size="sm"
          />
        )}
      </CardHeader>
      <CardContent
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
                disabled || gameOver ? "cursor-not-allowed" : "cursor-pointer"
              }`}
              onClick={() => handleCellClick(index)}
              disabled={disabled || gameOver}
            >
              {cell}
            </Button>
          ))}
        </div>
      </CardContent>
    </Card>
  );
}
