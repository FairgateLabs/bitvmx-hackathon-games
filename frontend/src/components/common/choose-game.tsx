"use client";

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

export function ChooseGame() {
  return (
    <div className="container mx-auto p-6 max-w-4xl">
      <div className="text-center mb-12">
        <div className="flex items-center justify-center mb-4">
          <h1 className="text-5xl font-bold">BitVMX Game 🎮</h1>
        </div>
        <p className="text-xl text-muted-foreground">
          Dive into the BitVMX protocol by playing the Add Numbers game.
        </p>
      </div>

      <div className="flex justify-center mb-12">
        <Card className="hover:shadow-lg transition-shadow max-w-md">
          <CardHeader>
            <CardTitle className="text-2xl">🔢 Add Numbers</CardTitle>
          </CardHeader>
          <CardContent>
            <CardDescription>
              A simple two-player game with numbers. <br />
              Player 1 gives two numbers. <br />
              Player 2 sums them. Player 1 confirms the answer — truthfully or
              by lying.
            </CardDescription>
          </CardContent>
          <CardFooter className="flex justify-center">
            <Link href="/add-numbers/play-game" className="w-full">
              <Button className="w-full" onClick={() => {}}>
                Play Now!
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
