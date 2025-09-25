"use client";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

export default function HowToPlayPage() {
  return (
    <div className="container mx-auto p-6 max-w-5xl">
      <Card className="bg-muted/40 shadow-lg">
        <CardHeader>
          <CardTitle className="text-2xl flex items-center gap-2">
            📚 How to Play <span className="text-primary">Add Numbers</span>
          </CardTitle>
          <p className="text-sm text-muted-foreground">
            Learn the game step by step. Player 1 and Player 2 follow different
            flows. This guide mirrors the exact in‑app flow and on‑chain steps
            ⚡
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
            <h3 className="text-lg font-semibold mb-2">🌐 Network & Wallet</h3>
            <ul className="list-disc list-inside text-sm text-muted-foreground space-y-1">
              <li>
                The game runs on <strong>Regtest</strong>.
              </li>
              <li>
                Wallets are auto‑funded by the local node. No manual funding is
                required.
              </li>
            </ul>
          </section>

          {/* Step 3 */}
          <section>
            <h3 className="text-lg font-semibold mb-2">
              🔌 Participant Data Exchange
            </h3>
            <p className="text-sm text-muted-foreground pb-2">
              Both players exchange connection information through two panels:
              <strong> Your Participant Data</strong> (shows your info) and
              <strong> Other Player's Participant Data</strong> (paste their
              info). The exchanged <strong>Participant Data</strong> consists
              of:
              <em> Aggregated Id, Public Key, Network Address, Peer ID</em>{" "}
              (Player 1 provides the Aggregated Id).
            </p>
            <div className="grid md:grid-cols-2 gap-6">
              <div className="p-4 rounded-xl bg-white shadow-sm border">
                <h4 className="font-semibold mb-2">👤 Player 1</h4>
                <ul className="list-disc list-inside text-sm text-muted-foreground space-y-1">
                  <li>
                    Select <strong>Player 1</strong> role. The app generates a
                    unique <strong>Aggregated Id</strong>.
                  </li>
                  <li>
                    <strong>Your Participant Data</strong> panel shows: Game
                    UUID, Public Key, Network Address, Peer ID.
                  </li>
                  <li>
                    Click <strong>"Copy to Share"</strong> and send this JSON to
                    Player 2.
                  </li>
                  <li>
                    In <strong>Other Player's Participant Data</strong>, paste
                    Player 2's JSON (Public Key, Network Address, Peer ID only).
                  </li>
                  <li>
                    Click <strong>"Setup Data"</strong> to establish connection.
                  </li>
                </ul>
              </div>
              <div className="p-4 rounded-xl bg-white shadow-sm border">
                <h4 className="font-semibold mb-2">👤 Player 2</h4>
                <ul className="list-disc list-inside text-sm text-muted-foreground space-y-1">
                  <li>
                    Select <strong>Player 2</strong> role.
                  </li>
                  <li>
                    <strong>Your Participant Data</strong> panel shows: Public
                    Key, Network Address, Peer ID.
                  </li>
                  <li>
                    Click <strong>"Copy to Share"</strong> and send this JSON to
                    Player 1.
                  </li>
                  <li>
                    In <strong>Other Player's Participant Data</strong>, paste
                    Player 1's JSON (Aggregated Id, Public Key, Network Address,
                    Peer ID).
                  </li>
                  <li>
                    Click <strong>"Setup Data"</strong> to establish connection.
                  </li>
                </ul>
              </div>
            </div>
          </section>

          {/* Step 4 */}
          <section>
            <h3 className="text-lg font-semibold mb-2">
              🔑 Aggregated Key Generation
            </h3>
            <p className="text-sm text-muted-foreground">
              To setup the game, both players exchange{" "}
              <strong>Participant Data</strong> (Aggregated ID, Public Key,
              Network Address, Peer ID) and the app derives an
              <strong> aggregated key</strong> that is displayed to both.
              <br /> In this part of the flow, both participants contact each
              other via BitVMX and generate an aggregated key that will be used
              to sign the whole protocol.
            </p>
          </section>

          {/* Step 5 */}
          <section>
            <h3 className="text-lg font-semibold mb-2">💵 Bet & Funding</h3>
            <div className="grid md:grid-cols-2 gap-6">
              <div className="p-4 rounded-xl bg-white shadow-sm border">
                <h4 className="font-semibold mb-2">👤 Player 1</h4>
                <ul className="list-disc list-inside text-sm text-muted-foreground space-y-1">
                  <li>
                    Accept the bet: <strong>0.0001 BTC</strong> + protocol fee.
                  </li>
                  <li>
                    For this demo, to simplify the experience,{" "}
                    <strong>only Player 1 funds</strong> the game. Player 2 pays
                    nothing.
                  </li>
                  <li>
                    The backend sends a funding transaction to the aggregated
                    wallet.
                  </li>
                  <li>
                    Generate the <strong>Funding UTXOs</strong> (protocol and
                    bet) and use <strong>Copy to Share</strong> to share the
                    JSON with Player 2.
                  </li>
                </ul>
              </div>
              <div className="p-4 rounded-xl bg-white shadow-sm border">
                <h4 className="font-semibold mb-2">👤 Player 2</h4>
                <ul className="list-disc list-inside text-sm text-muted-foreground space-y-1">
                  <li>No payment required in this demo.</li>
                  <li>Wait while Player 1 funds the game.</li>
                  <li>
                    When Player 1 shares the <strong>Funding UTXOs</strong>{" "}
                    JSON,
                    <strong> paste</strong> it into your UTXO form and submit to
                    continue.
                  </li>
                </ul>
              </div>
            </div>
          </section>

          {/* Step 6 */}
          <section>
            <h3 className="text-lg font-semibold mb-2">🎲 Game Flow</h3>

            <div className="grid md:grid-cols-2 gap-6">
              {/* Player 1 */}
              <div className="p-4 rounded-xl bg-white shadow-sm border">
                <h4 className="font-semibold mb-2">👤 Player 1</h4>
                <ol className="list-decimal list-inside text-sm text-muted-foreground space-y-1">
                  <li>
                    Enter two numbers and click{" "}
                    <strong>Generate Program</strong>.
                  </li>
                  <li>
                    Click <strong>Start Game</strong> to submit the first
                    on‑chain transaction.
                  </li>
                  <li>
                    The timer starts for Player 2. If Player 2 doesn’t respond
                    in time, you win by timeout.
                  </li>
                  <li>
                    When Player 2 submits an answer, review it and either{" "}
                    <strong>Accept</strong> (they win) or{" "}
                    <strong>Challenge</strong> (opens an on‑chain dispute to
                    prove the truth).
                  </li>
                </ol>
              </div>

              {/* Player 2 */}
              <div className="p-4 rounded-xl bg-white shadow-sm border">
                <h4 className="font-semibold mb-2">👤 Player 2</h4>
                <ol className="list-decimal list-inside text-sm text-muted-foreground space-y-1">
                  <li>
                    Enter the Aggregated Id from Player 1 and mirror the same
                    two numbers.
                  </li>
                  <li>Wait until Player 1 starts the game.</li>
                  <li>
                    Submit your <strong>sum answer</strong> on‑chain.
                  </li>
                  <li>
                    If Player 1 challenges, respond on‑chain before the timeout
                    to avoid losing.
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
                <strong>✅ Case 1:</strong> Player 2’s answer is correct →
                Player 2 wins on‑chain and receives the funds.
              </p>

              <p>
                <strong>❌ Case 2:</strong> Player 2’s answer is incorrect or
                Player 2 timed out to answer → Player 1 wins on‑chain and
                receives the funds.
              </p>
            </div>
          </section>
        </CardContent>
      </Card>
    </div>
  );
}
