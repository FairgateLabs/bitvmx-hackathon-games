"use client";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

export default function HowToPlayPage() {
  return (
    <div className="container mx-auto p-6 max-w-4xl">
      <Card className="bg-muted/50">
        <CardHeader>
          <CardTitle className="text-xl">📚 How to Play</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
            <div>
              <h4 className="font-semibold mb-2">🎯 Player 1</h4>
              <ul className="space-y-1 text-muted-foreground">
                <li>• Choose two numbers</li>
                <li>• Generate the program</li>
                <li>• Share the Game ID</li>
                <li>• Wait for the response</li>
              </ul>
            </div>

            <div>
              <h4 className="font-semibold mb-2">🎮 Player 2</h4>
              <ul className="space-y-1 text-muted-foreground">
                <li>• Enter the Game ID</li>
                <li>• Connect with Player 1</li>
                <li>• Calculate the sum</li>
                <li>• Send your response</li>
              </ul>
            </div>

            <div>
              <h4 className="font-semibold mb-2">⚖️ Resolution</h4>
              <ul className="space-y-1 text-muted-foreground">
                <li>• Player 1 verifies</li>
                <li>• Accept or dispute</li>
                <li>• Funds are transferred</li>
                <li>• Win or lose!</li>
              </ul>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
