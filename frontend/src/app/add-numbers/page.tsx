"use client";

import { useState } from "react";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Separator } from "@/components/ui/separator";
import { useAddNumbersGame } from "@/hooks/useAddNumbersGame";

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

export default function AddNumbersPage() {
  const [gameMode, setGameMode] = useState<"player1" | "player2" | null>(null);
  const [gameId, setGameId] = useState("");
  const [peerIP, setPeerIP] = useState("");
  const [peerPort, setPeerPort] = useState("");
  
  const {
    walletInfo,
    gameState,
    numbers,
    setNumbers,
    generateProgram,
    submitAnswer,
    acceptAnswer,
    challengeAnswer,
    isLoading,
    error
  } = useAddNumbersGame();

  if (!gameMode) {
    return (
      <div className="container mx-auto p-6 max-w-2xl">
        <Card>
          <CardHeader>
            <CardTitle className="text-2xl text-center">🎮 BitVMX Hackathon</CardTitle>
            <CardDescription className="text-center">
              Elige el juego que deseas jugar
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="text-center mb-6">
              <h2 className="text-xl font-semibold mb-2">Add Numbers Game</h2>
              <p className="text-muted-foreground">
                Dos jugadores compiten sumando números. ¿Quién eres?
              </p>
            </div>
            
            <div className="grid grid-cols-2 gap-4">
              <Button 
                onClick={() => setGameMode("player1")}
                className="h-24 text-lg"
                variant="outline"
              >
                🎯 Player 1<br/>
                <span className="text-sm font-normal">Crea el juego</span>
              </Button>
              
              <Button 
                onClick={() => setGameMode("player2")}
                className="h-24 text-lg"
                variant="outline"
              >
                🎮 Player 2<br/>
                <span className="text-sm font-normal">Únete al juego</span>
              </Button>
            </div>
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="container mx-auto p-6 max-w-4xl">
      <Card>
        <CardHeader>
          <CardTitle className="text-2xl">
            {gameMode === "player1" ? "🎯 Player 1 - Add Numbers" : "🎮 Player 2 - Add Numbers"}
          </CardTitle>
          <CardDescription>
            {gameMode === "player1" 
              ? "Crea el juego y elige los números a sumar" 
              : "Únete al juego y adivina la suma"
            }
          </CardDescription>
        </CardHeader>
        
        <CardContent className="space-y-6">
          {/* Wallet Information */}
          <WalletSection walletInfo={walletInfo} />
          
          <Separator />
          
          {/* Game Setup */}
          {gameMode === "player1" ? (
            <Player1GameSetup 
              numbers={numbers}
              setNumbers={setNumbers}
              generateProgram={generateProgram}
              gameId={gameId}
              gameState={gameState}
              isLoading={isLoading}
            />
          ) : (
            <Player2GameSetup 
              gameId={gameId}
              setGameId={setGameId}
              peerIP={peerIP}
              setPeerIP={setPeerIP}
              peerPort={peerPort}
              setPeerPort={setPeerPort}
              submitAnswer={submitAnswer}
              isLoading={isLoading}
            />
          )}
          
          {/* Game Actions */}
          {gameState === "waiting_response" && gameMode === "player1" && (
            <GameActions 
              onAccept={acceptAnswer}
              onChallenge={challengeAnswer}
              isLoading={isLoading}
            />
          )}
          
          {/* Error Display */}
          {error && (
            <div className="p-4 bg-destructive/10 border border-destructive rounded-md">
              <p className="text-destructive">{error}</p>
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}

function WalletSection({ walletInfo }: { walletInfo: WalletInfo | null }) {
  if (!walletInfo) return null;
  
  return (
    <div className="p-4 bg-muted/50 rounded-lg">
      <h3 className="font-semibold mb-2">💰 Wallet Information</h3>
      <div className="grid grid-cols-2 gap-4 text-sm">
        <div>
          <Label>Address:</Label>
          <p className="font-mono text-xs break-all">{walletInfo.address}</p>
        </div>
        <div>
          <Label>Balance:</Label>
          <p className="font-semibold">{walletInfo.balance} BTC</p>
        </div>
      </div>
    </div>
  );
}

interface Player1GameSetupProps {
  numbers: GameNumbers;
  setNumbers: (numbers: GameNumbers) => void;
  generateProgram: () => void;
  gameId: string;
  gameState: string;
  isLoading: boolean;
}

function Player1GameSetup({ 
  numbers, 
  setNumbers, 
  generateProgram, 
  gameId, 
  gameState, 
  isLoading 
}: Player1GameSetupProps) {
  return (
    <div className="space-y-4">
      <h3 className="text-lg font-semibold">🎲 Setup del Juego</h3>
      
      {!gameId ? (
        <div className="space-y-4">
          <div className="grid grid-cols-2 gap-4">
            <div>
              <Label htmlFor="number1">Primer Número</Label>
              <Input
                id="number1"
                type="number"
                value={numbers.number1 || ""}
                onChange={(e) => setNumbers({ ...numbers, number1: parseInt(e.target.value) })}
                placeholder="Ej: 5"
              />
            </div>
            <div>
              <Label htmlFor="number2">Segundo Número</Label>
              <Input
                id="number2"
                type="number"
                value={numbers.number2 || ""}
                onChange={(e) => setNumbers({ ...numbers, number2: parseInt(e.target.value) })}
                placeholder="Ej: 3"
              />
            </div>
          </div>
          
          <Button 
            onClick={generateProgram}
            disabled={!numbers.number1 || !numbers.number2 || isLoading}
            className="w-full"
          >
            {isLoading ? "Generando..." : "🚀 Generar Programa"}
          </Button>
        </div>
      ) : (
        <div className="p-4 bg-green-50 border border-green-200 rounded-md">
          <h4 className="font-semibold text-green-800 mb-2">✅ Programa Generado</h4>
          <p className="text-sm text-green-700 mb-2">Game ID:</p>
          <p className="font-mono text-xs bg-green-100 p-2 rounded break-all">{gameId}</p>
          <p className="text-sm text-green-700 mt-2">
            Comparte este ID con el Player 2 para que se una al juego.
          </p>
        </div>
      )}
      
      {gameState === "waiting_response" && (
        <div className="p-4 bg-blue-50 border border-blue-200 rounded-md">
          <h4 className="font-semibold text-blue-800">⏳ Esperando respuesta del Player 2...</h4>
          <p className="text-sm text-blue-700">
            El Player 2 está calculando la suma de {numbers.number1} + {numbers.number2}
          </p>
        </div>
      )}
    </div>
  );
}

interface Player2GameSetupProps {
  gameId: string;
  setGameId: (id: string) => void;
  peerIP: string;
  setPeerIP: (ip: string) => void;
  peerPort: string;
  setPeerPort: (port: string) => void;
  submitAnswer: (answer: string) => void;
  isLoading: boolean;
}

function Player2GameSetup({ 
  gameId, 
  setGameId, 
  peerIP, 
  setPeerIP, 
  peerPort, 
  setPeerPort, 
  submitAnswer, 
  isLoading 
}: Player2GameSetupProps) {
  const [answer, setAnswer] = useState("");
  
  return (
    <div className="space-y-4">
      <h3 className="text-lg font-semibold">🔗 Unirse al Juego</h3>
      
      {!gameId ? (
        <div className="space-y-4">
          <div>
            <Label htmlFor="gameId">Game ID</Label>
            <Input
              id="gameId"
              value={gameId}
              onChange={(e) => setGameId(e.target.value)}
              placeholder="Ingresa el UUID del juego"
            />
          </div>
          
          <div className="grid grid-cols-2 gap-4">
            <div>
              <Label htmlFor="peerIP">IP del Player 1</Label>
              <Input
                id="peerIP"
                value={peerIP}
                onChange={(e) => setPeerIP(e.target.value)}
                placeholder="Ej: 192.168.1.100"
              />
            </div>
            <div>
              <Label htmlFor="peerPort">Puerto</Label>
              <Input
                id="peerPort"
                value={peerPort}
                onChange={(e) => setPeerPort(e.target.value)}
                placeholder="Ej: 8080"
              />
            </div>
          </div>
          
          <Button 
            onClick={() => {/* TODO: Implement join game */}}
            disabled={!gameId || !peerIP || !peerPort}
            className="w-full"
          >
            🎮 Unirse al Juego
          </Button>
        </div>
      ) : (
        <div className="space-y-4">
          <div className="p-4 bg-blue-50 border border-blue-200 rounded-md">
            <h4 className="font-semibold text-blue-800">🎯 Calcula la Suma</h4>
            <p className="text-sm text-blue-700">
              El Player 1 ha elegido dos números. ¿Cuál es la suma?
            </p>
          </div>
          
          <div>
            <Label htmlFor="answer">Tu Respuesta</Label>
            <Input
              id="answer"
              type="number"
              value={answer}
              onChange={(e) => setAnswer(e.target.value)}
              placeholder="Ingresa la suma"
            />
          </div>
          
          <Button 
            onClick={() => submitAnswer(answer)}
            disabled={!answer || isLoading}
            className="w-full"
          >
            {isLoading ? "Enviando..." : "📤 Enviar Respuesta"}
          </Button>
        </div>
      )}
    </div>
  );
}

interface GameActionsProps {
  onAccept: () => void;
  onChallenge: () => void;
  isLoading: boolean;
}

function GameActions({ onAccept, onChallenge, isLoading }: GameActionsProps) {
  return (
    <div className="space-y-4">
      <h3 className="text-lg font-semibold">🎯 Acciones del Juego</h3>
      <p className="text-sm text-muted-foreground">
        El Player 2 ha enviado su respuesta. ¿Qué quieres hacer?
      </p>
      
      <div className="grid grid-cols-2 gap-4">
        <Button 
          onClick={onAccept}
          disabled={isLoading}
          className="bg-green-600 hover:bg-green-700"
        >
          ✅ Correcto - Transferir Fondos
        </Button>
        
        <Button 
          onClick={onChallenge}
          disabled={isLoading}
          variant="destructive"
        >
          ⚖️ Challenge - Iniciar Disputa
        </Button>
      </div>
      
      <p className="text-xs text-muted-foreground text-center">
        ⏰ Si no haces nada, el Player 2 ganará automáticamente por timeout
      </p>
    </div>
  );
}
