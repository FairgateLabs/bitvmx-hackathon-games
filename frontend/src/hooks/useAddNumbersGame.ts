import { useState, useCallback } from "react";

// Types
interface WalletInfo {
  address: string;
  balance: number;
  network: "regtest" | "testnet";
}

interface GameNumbers {
  number1?: number;
  number2?: number;
}

type GameState = 
  | "setup"
  | "waiting_peer"
  | "waiting_response"
  | "response_received"
  | "completed"
  | "disputed"
  | "timeout";

interface GameResponse {
  answer: number;
  isCorrect: boolean;
  timestamp: string;
}

// Hook
export const useAddNumbersGame = () => {
  // State
  const [walletInfo, setWalletInfo] = useState<WalletInfo | null>(null);
  const [gameState, setGameState] = useState<GameState>("setup");
  const [numbers, setNumbers] = useState<GameNumbers>({});
  const [gameId, setGameId] = useState<string>("");
  const [gameResponse, setGameResponse] = useState<GameResponse | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Initialize wallet (simulated for now)
  const initializeWallet = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);
      
      // Simulate API call to get wallet info
      // In real implementation, this would call your backend
      const mockWalletInfo: WalletInfo = {
        address: "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
        balance: 1.0, // 1 BTC for regtest
        network: "regtest"
      };
      
      setWalletInfo(mockWalletInfo);
      setGameState("setup");
    } catch (err) {
      setError("Error initializing wallet: " + (err as Error).message);
    } finally {
      setIsLoading(false);
    }
  }, []);

  // Generate program (Player 1)
  const generateProgram = useCallback(async () => {
    if (!numbers.number1 || !numbers.number2) {
      setError("Please enter both numbers");
      return;
    }

    try {
      setIsLoading(true);
      setError(null);
      
      // Simulate API call to generate program
      // In real implementation, this would call your backend
      await new Promise(resolve => setTimeout(resolve, 2000)); // Simulate delay
      
      const newGameId = `game_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
      setGameId(newGameId);
      setGameState("waiting_peer");
      
      // Simulate peer joining
      setTimeout(() => {
        setGameState("waiting_response");
      }, 3000);
      
    } catch (err) {
      setError("Error generating program: " + (err as Error).message);
    } finally {
      setIsLoading(false);
    }
  }, [numbers]);

  // Submit answer (Player 2)
  const submitAnswer = useCallback(async (answer: string) => {
    if (!answer) {
      setError("Please enter your answer");
      return;
    }

    try {
      setIsLoading(true);
      setError(null);
      
      const answerNum = parseInt(answer);
      const correctSum = (numbers.number1 || 0) + (numbers.number2 || 0);
      const isCorrect = answerNum === correctSum;
      
      // Simulate API call to submit answer
      // In real implementation, this would call your backend
      await new Promise(resolve => setTimeout(resolve, 1500)); // Simulate delay
      
      const response: GameResponse = {
        answer: answerNum,
        isCorrect,
        timestamp: new Date().toISOString()
      };
      
      setGameResponse(response);
      setGameState("response_received");
      
      // Simulate Player 1 receiving the response
      setTimeout(() => {
        setGameState("waiting_response");
      }, 1000);
      
    } catch (err) {
      setError("Error submitting answer: " + (err as Error).message);
    } finally {
      setIsLoading(false);
    }
  }, [numbers]);

  // Accept answer (Player 1)
  const acceptAnswer = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);
      
      // Simulate API call to accept answer
      // In real implementation, this would call your backend
      await new Promise(resolve => setTimeout(resolve, 2000)); // Simulate delay
      
      setGameState("completed");
      
      // Show success message
      alert("✅ Respuesta correcta! Los fondos han sido transferidos al Player 2.");
      
    } catch (err) {
      setError("Error accepting answer: " + (err as Error).message);
    } finally {
      setIsLoading(false);
    }
  }, []);

  // Challenge answer (Player 1)
  const challengeAnswer = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);
      
      // Simulate API call to challenge answer
      // In real implementation, this would call your backend
      await new Promise(resolve => setTimeout(resolve, 2000)); // Simulate delay
      
      setGameState("disputed");
      
      // Show challenge message
      alert("⚖️ Disputa iniciada. El Player 2 tiene tiempo para responder.");
      
    } catch (err) {
      setError("Error challenging answer: " + (err as Error).message);
    } finally {
      setIsLoading(false);
    }
  }, []);

  // Join game (Player 2)
  const joinGame = useCallback(async (gameId: string, peerIP: string, peerPort: string) => {
    if (!gameId || !peerIP || !peerPort) {
      setError("Please fill in all fields");
      return;
    }

    try {
      setIsLoading(true);
      setError(null);
      
      // Simulate API call to join game
      // In real implementation, this would call your backend
      await new Promise(resolve => setTimeout(resolve, 1500)); // Simulate delay
      
      setGameId(gameId);
      setGameState("waiting_peer");
      
      // Simulate receiving game numbers
      setTimeout(() => {
        setNumbers({ number1: 5, number2: 3 }); // Mock numbers
        setGameState("waiting_response");
      }, 2000);
      
    } catch (err) {
      setError("Error joining game: " + (err as Error).message);
    } finally {
      setIsLoading(false);
    }
  }, []);

  // Reset game
  const resetGame = useCallback(() => {
    setGameState("setup");
    setNumbers({});
    setGameId("");
    setGameResponse(null);
    setError(null);
  }, []);

  // Auto-initialize wallet when hook is used
  useState(() => {
    initializeWallet();
  });

  return {
    // State
    walletInfo,
    gameState,
    numbers,
    gameId,
    gameResponse,
    isLoading,
    error,
    
    // Actions
    setNumbers,
    generateProgram,
    submitAnswer,
    acceptAnswer,
    challengeAnswer,
    joinGame,
    resetGame,
    initializeWallet
  };
};
