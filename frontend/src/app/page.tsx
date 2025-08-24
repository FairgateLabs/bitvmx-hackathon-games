import Link from "next/link";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

export default function Home() {
  return (
    <div className="container mx-auto p-6 max-w-4xl">
      <div className="text-center mb-12">
        <div className="flex items-center justify-center mb-4">
          <h1 className="text-5xl font-bold">BitVMX Games 🎮</h1>
        </div>
        <p className="text-xl text-muted-foreground">
          Experience the BitVMX protocol by playing and learning with different
          games. Challenge the results if you disagree with the outcome and
          compete for Bitcoins!
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-12">
        <Card className="hover:shadow-lg transition-shadow">
          <CardHeader>
            <CardTitle className="text-2xl">🔢 Add Numbers</CardTitle>
          </CardHeader>
          <CardContent>
            <CardDescription>
              Simple game where two players compete by adding numbers. <br />
              Player 1 chooses two numbers, Player 2 must guess the sum.
            </CardDescription>
          </CardContent>
          <CardFooter className="flex justify-center">
            <Link href="/add-numbers/play-game" className="w-full">
              <Button className="w-full cursor-pointer">
                🎯 Play for 1 BTC in Regtest!{" "}
              </Button>
            </Link>
          </CardFooter>
        </Card>

        <Card className="hover:shadow-lg transition-shadow opacity-60">
          <CardHeader>
            <CardTitle className="text-2xl">⭕ Tic Tac Toe</CardTitle>
          </CardHeader>
          <CardContent>
            <CardDescription>
              Classic Tic Tac Toe game using BitVMX protocol <br />
              Player 1 and Player 2 take turns marking spaces in a 3×3 grid.{" "}
              <br /> The first to align three marks wins.
            </CardDescription>
          </CardContent>
          <CardFooter className="flex justify-center">
            <Link href="/tic-tac-toe/play-game" className="w-full">
              <Button className="w-full" disabled>
                🚧 Coming Soon...
              </Button>
            </Link>
          </CardFooter>
        </Card>
      </div>

      <div className="text-center mt-8 text-sm text-muted-foreground p-6">
        <p>
          🌟 Created for the Bitcoin Hackathon in Berlin, crafted with
          dedication and enthusiasm by our team!
        </p>
      </div>
    </div>
  );
}
