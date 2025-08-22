import Link from "next/link";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

export default function Home() {
  return (
    <div className="container mx-auto p-6 max-w-4xl">
      <div className="text-center mb-12">
        <h1 className="text-5xl font-bold mb-4">🎮 BitVMX Games</h1>
        <p className="text-xl text-muted-foreground">
          Bitcoin Hackathon in Berlin - Try our BitVMX protocol
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-12">
        <Card className="hover:shadow-lg transition-shadow">
          <CardHeader>
            <CardTitle className="text-2xl">🔢 Add Numbers</CardTitle>
            <CardDescription>
              Simple game where two players compete by adding numbers
            </CardDescription>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-muted-foreground mb-4">
              Player 1 chooses two numbers, Player 2 must guess the sum. Play
              for 1 BTC in Regtest!
            </p>
            <Link href="/add-numbers">
              <Button className="w-full">🎯 Play Add Numbers</Button>
            </Link>
          </CardContent>
        </Card>

        <Card className="hover:shadow-lg transition-shadow opacity-60">
          <CardHeader>
            <CardTitle className="text-2xl">⭕ Tic Tac Toe</CardTitle>
            <CardDescription>
              Classic tic-tac-toe game with bets
            </CardDescription>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-muted-foreground mb-4">
              Coming soon... Tic Tac Toe game with BitVMX protocol
            </p>
            <Button className="w-full" disabled>
              🚧 Coming Soon
            </Button>
          </CardContent>
        </Card>
      </div>

      <div className="text-center mt-8 text-sm text-muted-foreground">
        <p>
          🚀 Developed for the Bitcoin Hackathon in Berlin with BitVMX protocol
        </p>
      </div>
    </div>
  );
}
