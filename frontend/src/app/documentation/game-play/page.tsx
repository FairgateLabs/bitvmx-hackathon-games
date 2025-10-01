export default function GamePlayPage() {
  return (
    <div className="mx-auto max-w-5xl px-4 py-10">
      <h1 className="text-3xl font-bold tracking-tight">
        Add Numbers Game Play
      </h1>
      <p className="mt-4 text-muted-foreground">
        This document explains the complete flow of the Add Numbers game, a
        BitVMX-based game where two players compete to solve a mathematical
        challenge. The game uses BitVMX's dispute resolution protocol to ensure
        fair play and the Bitcoin network for decentralized immutable
        transactions.
      </p>

      <section className="mt-10">
        <h2 className="text-2xl font-semibold">Players</h2>
        <p className="mt-3 text-muted-foreground">
          Player 1 will be using the front end at{" "}
          <code className="rounded bg-muted px-1 py-0.5">
            http://localhost:3000
          </code>
          , and player 2 will be using{" "}
          <code className="rounded bg-muted px-1 py-0.5">
            http://localhost:3001
          </code>
        </p>
      </section>

      <section className="mt-10">
        <h2 className="text-2xl font-semibold">Game Play Steps</h2>

        <article className="mt-6 space-y-4">
          <h3 className="text-xl font-semibold">
            Step 1: Select the Add Numbers Game
          </h3>
          <p className="text-muted-foreground">
            The player should select the game to play. This project only has the
            Add Numbers game for the moment, but we could add more games later
            on.
          </p>
          <div className="mt-4 overflow-hidden rounded-lg border bg-background">
            <img
              src="/game-play/game-play-1.png"
              alt="Add Numbers Game selection"
              className="w-full"
            />
          </div>
        </article>

        <article className="mt-10 space-y-4">
          <h3 className="text-xl font-semibold">Step 2: Select the Network</h3>
          <p className="text-muted-foreground">
            The player selects the Bitcoin network. This game is meant to be
            used in regtest only for the moment.
          </p>
          <div className="mt-4 overflow-hidden rounded-lg border bg-background">
            <img
              src="/game-play/game-play-2.png"
              alt="Select Network"
              className="w-full"
            />
          </div>
        </article>

        <article className="mt-10 space-y-4">
          <h3 className="text-xl font-semibold">Step 3: Select Player</h3>
          <p className="text-muted-foreground">
            Each frontend instance can only select their assigned player.
            localhost:3000 can only select Player 1, and localhost:3001 can only
            select Player 2.
          </p>

          <div className="grid gap-6 md:grid-cols-2">
            <div>
              <h4 className="font-medium">
                Player 1 Selection (localhost:3000):
              </h4>
              <div className="mt-4 overflow-hidden rounded-lg border bg-background">
                <img
                  src="/game-play/game-play-3.png"
                  alt="Select Player 1"
                  className="w-full"
                />
              </div>
            </div>
            <div>
              <h4 className="font-medium">
                Player 2 Selection (localhost:3001):
              </h4>
              <div className="mt-4 overflow-hidden rounded-lg border bg-background">
                <img
                  src="/game-play/game-play-4.png"
                  alt="Select Player 2"
                  className="w-full"
                />
              </div>
            </div>
          </div>
        </article>

        <article className="mt-10 space-y-4">
          <h3 className="text-xl font-semibold">Step 4: Player Wallets</h3>
          <p className="text-muted-foreground">
            Each player will see their own wallet view with different Bitcoin
            addresses and current balance. When setting up the project, each
            player was given 100 BTC. From this initial balance, 1 BTC was sent
            to BitVMX for speed up fundings to be used as "child pays for
            parent" transactions.
          </p>

          <div className="grid gap-6 md:grid-cols-2">
            <div>
              <h4 className="font-medium">Player 1 Wallet View:</h4>
              <div className="mt-4 overflow-hidden rounded-lg border bg-background">
                <img
                  src="/game-play/game-play-5.png"
                  alt="Player 1 Wallet"
                  className="w-full"
                />
              </div>
            </div>
            <div>
              <h4 className="font-medium">Player 2 Wallet View:</h4>
              <div className="mt-4 overflow-hidden rounded-lg border bg-background">
                <img
                  src="/game-play/game-play-6.png"
                  alt="Player 2 Wallet"
                  className="w-full"
                />
              </div>
            </div>
          </div>
        </article>

        <article className="mt-10 space-y-4">
          <h3 className="text-xl font-semibold">
            Step 5: Participant Data Exchange
          </h3>
          <p className="text-muted-foreground">
            Before starting the game, players need to exchange participant data.
            Copy the data from Player 1 at localhost:3000 and paste it into
            Player 2 at localhost:3001, then copy the data from Player 2 and
            paste it into Player 1. After exchanging the data and clicking on
            "Setup Data", the aggregated key using MuSig2 is created.
          </p>

          <div className="grid gap-6 md:grid-cols-2">
            <div>
              <h4 className="font-medium">
                Copy Participant Data from Player 1:
              </h4>
              <div className="mt-4 overflow-hidden rounded-lg border bg-background">
                <img
                  src="/game-play/game-play-7.png"
                  alt="Participant Data Copy"
                  className="w-full"
                />
              </div>
            </div>
            <div>
              <h4 className="font-medium">
                Paste Participant Data to Player 2:
              </h4>
              <div className="mt-4 overflow-hidden rounded-lg border bg-background">
                <img
                  src="/game-play/game-play-8.png"
                  alt="Participant Data Paste"
                  className="w-full"
                />
              </div>
            </div>
          </div>
        </article>

        <article className="mt-10 space-y-4">
          <h3 className="text-xl font-semibold">
            Step 6: Player 1 Send Bet Transaction
          </h3>
          <p className="text-muted-foreground">
            Player 1 transfers 2 UTXOs to the aggregated key. The first UTXO is
            to pay for protocol fees, and the second UTXO is the bet.
          </p>
          <div className="mt-4 overflow-hidden rounded-lg border bg-background">
            <img
              src="/game-play/game-play-9.png"
              alt="Player 1 Send Bet Transaction"
              className="w-full"
            />
          </div>
        </article>

        <article className="mt-10 space-y-4">
          <h3 className="text-xl font-semibold">Step 7: Set Funding UTXO</h3>
          <p className="text-muted-foreground">
            The UTXOs obtained from step 6 need to be copied from Player 1 and
            given to Player 2, as both players need to feed the same information
            when setting up the program. Player 1 already has the UTXOs as it's
            the one that sent them.
          </p>

          <div className="grid gap-6 md:grid-cols-2">
            <div>
              <h4 className="font-medium">Copy UTXOs from Player 1:</h4>
              <div className="mt-4 overflow-hidden rounded-lg border bg-background">
                <img
                  src="/game-play/game-play-10.png"
                  alt="UTXO Exchange Copy"
                  className="w-full"
                />
              </div>
            </div>
            <div>
              <h4 className="font-medium">Paste UTXOs to Player 2:</h4>
              <div className="mt-4 overflow-hidden rounded-lg border bg-background">
                <img
                  src="/game-play/game-play-11.png"
                  alt="UTXO Exchange Paste"
                  className="w-full"
                />
              </div>
            </div>
          </div>
        </article>

        <article className="mt-10 space-y-4">
          <h3 className="text-xl font-semibold">Step 8: Game Setup</h3>
          <p className="text-muted-foreground">
            Both players need to enter the exact same numbers and click on
            "Generate Program". Otherwise, the program won't complete as the
            other participant won't sign it because they have different values.
          </p>
          <div className="mt-4 overflow-hidden rounded-lg border bg-background">
            <img
              src="/game-play/game-play-12.png"
              alt="Game Setup"
              className="w-full"
            />
          </div>
        </article>

        <article className="mt-10 space-y-4">
          <h3 className="text-xl font-semibold">Step 9: Start Game</h3>
          <p className="text-muted-foreground">
            Once the game is set up, Player 1 will click on "Start Game" to
            start the BitVMX dispute protocol, where Player 2 is the prover that
            needs to show the result of the program. This is done by BitVMX with
            a start challenge transaction broadcasted to the Bitcoin network. In
            the meantime, Player 2 is waiting for Player 1 to start the game by
            watching for the challenge broadcasted transaction.
          </p>

          <div className="grid gap-6 md:grid-cols-2">
            <div>
              <h4 className="font-medium">Ready to Start Game:</h4>
              <div className="mt-4 overflow-hidden rounded-lg border bg-background">
                <img
                  src="/game-play/game-play-13.png"
                  alt="Ready Start Game"
                  className="w-full"
                />
              </div>
            </div>
            <div>
              <h4 className="font-medium">
                Player 2 Waiting for Player 1 to Start Game:
              </h4>
              <div className="mt-4 overflow-hidden rounded-lg border bg-background">
                <img
                  src="/game-play/game-play-14.png"
                  alt="Wait Player 1 Start Game"
                  className="w-full"
                />
              </div>
            </div>
          </div>
        </article>

        <article className="mt-10 space-y-4">
          <h3 className="text-xl font-semibold">
            Step 10: Player 2 Submit Answer
          </h3>
          <p className="text-muted-foreground">
            Player 2 needs to enter the answer to the sum of the numbers entered
            in the setup program. This will be added as an input to the BitVMX
            program that will continue with the dispute protocol automatically.
            It takes several transactions to reach the result of the dispute. As
            this project auto mines blocks every 5 seconds, the dispute
            resolution takes about a minute.
          </p>

          <div className="grid gap-6 md:grid-cols-2">
            <div>
              <h4 className="font-medium">
                Player 1 Waits for Player 2 Submit Answer:
              </h4>
              <div className="mt-4 overflow-hidden rounded-lg border bg-background">
                <img
                  src="/game-play/game-play-15.png"
                  alt="Player 1 Waits for Player 2 Submit Answer"
                  className="w-full"
                />
              </div>
            </div>
            <div>
              <h4 className="font-medium">Player 2 Submit Answer:</h4>
              <div className="mt-4 overflow-hidden rounded-lg border bg-background">
                <img
                  src="/game-play/game-play-16.png"
                  alt="Player 2 Submit Answer"
                  className="w-full"
                />
              </div>
            </div>
          </div>
        </article>

        <article className="mt-10 space-y-4">
          <h3 className="text-xl font-semibold">Step 11: Game Result</h3>
          <p className="text-muted-foreground">
            BitVMX automatically executed the dispute protocol reaching a
            winner. If the sum entered is correct, then the winner is Player 2
            and Player 1 loses.
          </p>

          <div className="grid gap-6 md:grid-cols-2">
            <div>
              <h4 className="font-medium">Player 2 Wins:</h4>
              <div className="mt-4 overflow-hidden rounded-lg border bg-background">
                <img
                  src="/game-play/game-play-17.png"
                  alt="Player 2 Wins"
                  className="w-full"
                />
              </div>
            </div>
            <div>
              <h4 className="font-medium">Player 1 Loses:</h4>
              <div className="mt-4 overflow-hidden rounded-lg border bg-background">
                <img
                  src="/game-play/game-play-18.png"
                  alt="Player 1 Loses"
                  className="w-full"
                />
              </div>
            </div>
          </div>
        </article>
      </section>
    </div>
  );
}
