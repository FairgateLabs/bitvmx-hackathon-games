import Link from "next/link";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";

export default function Home() {
  return (
    <div className="container mx-auto p-6 max-w-4xl">
      <div className="text-center mb-12">
        <h1 className="text-5xl font-bold mb-4">🎮 BitVMX Games</h1>
        <p className="text-xl text-muted-foreground">
          Hackathon de Bitcoin en Berlín - Prueba nuestro protocolo BitVMX
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-12">
        <Card className="hover:shadow-lg transition-shadow">
          <CardHeader>
            <CardTitle className="text-2xl">🔢 Add Numbers</CardTitle>
            <CardDescription>
              Juego simple donde dos jugadores compiten sumando números
            </CardDescription>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-muted-foreground mb-4">
              El Player 1 elige dos números, el Player 2 debe adivinar la suma.
              ¡Juega por 1 BTC en Regtest!
            </p>
            <Link href="/add-numbers">
              <Button className="w-full">🎯 Jugar Add Numbers</Button>
            </Link>
          </CardContent>
        </Card>

        <Card className="hover:shadow-lg transition-shadow opacity-60">
          <CardHeader>
            <CardTitle className="text-2xl">⭕ Tic Tac Toe</CardTitle>
            <CardDescription>
              Clásico juego del tres en raya con apuestas
            </CardDescription>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-muted-foreground mb-4">
              Próximamente... Juego de Tic Tac Toe con protocolo BitVMX
            </p>
            <Button className="w-full" disabled>
              🚧 Próximamente
            </Button>
          </CardContent>
        </Card>
      </div>

      <Card className="bg-muted/50">
        <CardHeader>
          <CardTitle className="text-xl">📚 Cómo Jugar</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
            <div>
              <h4 className="font-semibold mb-2">🎯 Player 1</h4>
              <ul className="space-y-1 text-muted-foreground">
                <li>• Elige dos números</li>
                <li>• Genera el programa</li>
                <li>• Comparte el Game ID</li>
                <li>• Espera la respuesta</li>
              </ul>
            </div>
            
            <div>
              <h4 className="font-semibold mb-2">🎮 Player 2</h4>
              <ul className="space-y-1 text-muted-foreground">
                <li>• Ingresa el Game ID</li>
                <li>• Conecta con Player 1</li>
                <li>• Calcula la suma</li>
                <li>• Envía tu respuesta</li>
              </ul>
            </div>
            
            <div>
              <h4 className="font-semibold mb-2">⚖️ Resolución</h4>
              <ul className="space-y-1 text-muted-foreground">
                <li>• Player 1 verifica</li>
                <li>• Acepta o disputa</li>
                <li>• Fondos se transfieren</li>
                <li>• ¡Gana o pierde!</li>
              </ul>
            </div>
          </div>
        </CardContent>
      </Card>

      <div className="text-center mt-8 text-sm text-muted-foreground">
        <p>
          🚀 Desarrollado para la Hackathon de Bitcoin en Berlín con protocolo BitVMX
        </p>
      </div>
    </div>
  );
}
