"use client";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

export default function HowToPlayPage() {
  return (
    <div className="container mx-auto p-6 max-w-5xl">
      <Card className="bg-muted/40 shadow-lg">
        <CardHeader>
          <CardTitle className="text-2xl flex items-center gap-2">
            📚 How to Play <span className="text-primary">Tic Tac Toe</span>
          </CardTitle>
          <p className="text-sm text-muted-foreground">
            Learn the game step by step before betting with Bitcoin ⚡
          </p>
        </CardHeader>
        <CardContent className="space-y-8">
          {/* Step 1 */}
          <section>
            <h3 className="text-lg font-semibold mb-2">🎮 Enter the Game</h3>
            <p className="text-sm text-muted-foreground mb-3">
              Each player joins from their browser:
            </p>
            <ul className="list-disc list-inside text-sm text-muted-foreground space-y-1">
              <li>
                Player 1 → <code>localhost:3000</code>
              </li>
              <li>
                Player 2 → <code>localhost:3001</code>
              </li>
            </ul>
            <p className="text-sm text-muted-foreground mt-2">
              Both players must select the same game to start.
            </p>
          </section>

          {/* Step 2 */}
          <section>
            <h3 className="text-lg font-semibold mb-2">💰 Wallet Funding</h3>
            <ul className="list-disc list-inside text-sm text-muted-foreground space-y-1">
              <li>
                <strong>Regtest:</strong> the wallet is automatically funded.
              </li>
              <li>
                <strong>Testnet:</strong> send funds to the displayed address,
                then enter the <code>txid</code> and <code>UTXO</code>, and
                confirm.
              </li>
            </ul>
          </section>

          {/* Step 3 */}
          <section>
            <h3 className="text-lg font-semibold mb-2">
              🆔 Game Identification
            </h3>
            <p className="text-sm text-muted-foreground">
              Player 1 creates the <strong>Game UUID</strong> (program ID).
            </p>
            <p className="text-sm text-muted-foreground">
              {" "}
              Player 2 must enter:
            </p>
            <ul className="list-disc list-inside text-sm text-muted-foreground mt-2">
              <li>The Game UUID</li>
              <li>Player 1’s IP address</li>
              <li>The corresponding port</li>
            </ul>
          </section>

          {/* Step 4 */}
          <section>
            <h3 className="text-lg font-semibold mb-2">
              🔑 Shared Key Generation
            </h3>
            <p className="text-sm text-muted-foreground">
              Both clients exchange wallets information, agree, and generate an{" "}
              <strong>aggregated key</strong> that is shown to both players.
            </p>
          </section>

          {/* Step 5 */}
          <section>
            <h3 className="text-lg font-semibold mb-2">💵 Initial Bet</h3>
            <p className="text-sm text-muted-foreground">
              The bet is automatically set:
            </p>
            <ul className="list-disc list-inside text-sm text-muted-foreground">
              <li>1 BTC in Regtest</li>
              <li>0.001 BTC in Testnet</li>
            </ul>
          </section>

          {/* Step 6 */}
          <section>
            <h3 className="text-lg font-semibold mb-2">🎲 Game Flow</h3>

            <div className="grid md:grid-cols-2 gap-6d">
              {/* Player 1 */}
              <div className="p-4 rounded-xl bg-white shadow-sm border">
                <h4 className="font-semibold mb-2">👤 Player 1</h4>
                <ol className="list-decimal list-inside text-sm text-muted-foreground space-y-1">
                  <li>
                    Mark with mark X on the grid, you can overwrite any previous
                    mark by Player 2.
                  </li>
                  <li>
                    Declare &quot;I win&quot; if you believe you have a winning
                    line.
                  </li>
                  <li>
                    If you don’t act before the timeout ⏳, you automatically
                    lose.
                  </li>
                  <li>
                    If challenged, wait for BitVMX process the dispute and
                    receive the result.
                  </li>
                </ol>
              </div>

              <div className="ml-2 p-4 rounded-xl bg-white shadow-sm border">
                <h4 className="font-semibold mb-2">👤 Player 2</h4>
                <ol className="list-decimal list-inside text-sm text-muted-foreground space-y-1">
                  <li>
                    Mark with mark O on the grid, you can overwrite any previous
                    mark by Player 1.
                  </li>
                  <li>
                    Declare &quot;I win&quot; if you believe you have a winning
                    line.
                  </li>
                  <li>
                    If you don’t act before the timeout ⏳, you automatically
                    lose.
                  </li>
                  <li>
                    If challenged, wait for BitVMX process the dispute and
                    receive the result.
                  </li>
                </ol>
              </div>
            </div>
          </section>

          {/* Outcomes */}
          <section>
            <h3 className="text-lg font-semibold mb-2">⚖️ Possible Outcomes</h3>
            <div className="space-y-4 text-sm text-muted-foreground">
              <p>
                <strong>✅ Case 1:</strong> A player declares &quot;I win&quot;
                and the claim is correct → The declaring player receives the
                funds.
              </p>

              <p>
                <strong>⚔️ Case 2:</strong> A player declares &quot;I win&quot;
                and the claim is challenged. The truth is settled on-chain:
              </p>
              <ul className="list-disc list-inside ml-4 mt-1 space-y-1">
                <li>
                  If the claim was <strong>correct</strong>, the declaring
                  player wins on-chain and receives the funds.
                </li>
                <li>
                  If the claim was <strong>wrong</strong>, the challenging
                  player wins on-chain and wins the funds.
                </li>
                <li>
                  If the declaring player lied and challenged a correct
                  challenge, they will <strong>lose the dispute</strong>{" "}
                  on-chain and the challenging player wins the funds.
                </li>
              </ul>

              <p>
                <strong>⏳ Case 3:</strong> Timeout applies at every stage. If
                either player fails to act (declare, challenge, or respond to a
                challenge) before the deadline, that player{" "}
                <strong>automatically loses</strong> and the other player wins
                the funds.
              </p>
            </div>
          </section>
        </CardContent>
      </Card>
    </div>
  );
}
