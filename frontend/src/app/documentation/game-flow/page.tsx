export default function GameFlowPage() {
  return (
    <div className="mx-auto max-w-5xl px-4 py-10">
      <h1 className="text-3xl font-bold tracking-tight">
        Add Numbers Game Flow
      </h1>
      <p className="mt-4 text-muted-foreground">
        This document explains the complete flow of the Add Numbers game, a
        BitVMX-based game where two players compete to solve a mathematical
        challenge. The game uses BitVMX's dispute resolution protocol to ensure
        fair play and the Bitcoin network for decentralized immutable
        transactions.
      </p>

      <section className="mt-10">
        <h2 className="text-2xl font-semibold">Game Overview</h2>
        <ul className="mt-4 list-disc space-y-2 pl-6">
          <li>
            <strong>Player 1</strong> creates the game and provides two numbers
            to sum
          </li>
          <li>
            <strong>Player 2</strong> joins the game and tries to guess the
            correct sum
          </li>
          <li>
            The game uses BitVMX's dispute resolution protocol to verify the
            answer
          </li>
          <li>The winner automatically receives the bet funds</li>
        </ul>
      </section>

      <section className="mt-10">
        <h2 className="text-2xl font-semibold">BitVMX One-Time Setup</h2>
        <p className="mt-3 text-muted-foreground">
          Before any game can be played, both players must perform a one-time
          initialization process on their respective BitVMX nodes. This setup
          creates the necessary cryptographic keys, establishes P2P
          communication, and funds the system for Bitcoin operations.
        </p>
        <p className="mt-3 text-muted-foreground">
          The{" "}
          <code className="rounded bg-muted px-1 py-0.5">initial_setup()</code>{" "}
          method performs the following initialization steps:
        </p>
        <ol className="mt-4 list-decimal space-y-2 pl-6">
          <li>
            <strong>P2P Address Setup</strong> - Establishes peer-to-peer
            communication information for BitVMX protocol participation
          </li>
          <li>
            <strong>Wallet Address Setup</strong> - Creates and funds the main
            Bitcoin wallet (requires at least 1 BTC)
          </li>
          <li>
            <strong>Operator Key Creation</strong> - Generates the main
            cryptographic key for protocol operations
          </li>
          <li>
            <strong>Funding Key Creation</strong> - Sets up Child Pays for
            Parent (CPFP) speed-up capability (requires 1 BTC)
          </li>
        </ol>
        <p className="mt-3 text-muted-foreground">
          This one-time setup is essential for any BitVMX node to participate in
          protocols, handle Bitcoin transactions, and perform speed-up
          operations when needed. The setup must be completed before any game
          can be initiated.
        </p>
      </section>

      <section className="mt-10">
        <h2 className="text-2xl font-semibold">Game States</h2>
        <ol className="mt-4 list-decimal space-y-1 pl-6">
          <li>Setup Participants</li>
          <li>Place Bet</li>
          <li>Setup Funding</li>
          <li>Setup Game</li>
          <li>Start Game</li>
          <li>Submit Game Data</li>
          <li>Game Complete</li>
          <li>Finished</li>
        </ol>
      </section>

      <hr className="my-10 border-border" />

      <section className="mt-10">
        <h2 className="text-2xl font-semibold">Step-by-Step Game Flow</h2>

        <article className="mt-6 space-y-4">
          <h3 className="text-xl font-semibold">Step 1: Setup Participants</h3>
          <p className="text-muted-foreground">
            <strong>What happens:</strong> Both players first share their
            participant information, then each player submits all participants'
            information to create the aggregated key. If participant information
            differs between players, the aggregated key creation will fail.
          </p>
          <div className="grid gap-6 md:grid-cols-2">
            <div>
              <h4 className="font-medium">Player 1 Actions:</h4>
              <ul className="mt-2 list-disc space-y-2 pl-6">
                <li>
                  Copies Player 2's information and adds their own P2P address,
                  public key, and role as Player1
                </li>
                <li>
                  Calls{" "}
                  <code className="rounded bg-muted px-1 py-0.5">
                    /setup-participants
                  </code>{" "}
                  with complete participant information (both players)
                </li>
                <li>
                  Receives a{" "}
                  <code className="rounded bg-muted px-1 py-0.5">
                    program_id
                  </code>{" "}
                  and{" "}
                  <code className="rounded bg-muted px-1 py-0.5">
                    aggregated_key
                  </code>
                </li>
              </ul>
            </div>
            <div>
              <h4 className="font-medium">Player 2 Actions:</h4>
              <ul className="mt-2 list-disc space-y-2 pl-6">
                <li>
                  Copies Player 1's information and adds their own P2P address,
                  public key, and role as Player2
                </li>
                <li>
                  Calls{" "}
                  <code className="rounded bg-muted px-1 py-0.5">
                    /setup-participants
                  </code>{" "}
                  with the same{" "}
                  <code className="rounded bg-muted px-1 py-0.5">
                    aggregated_id
                  </code>{" "}
                  and complete participant information (both players)
                </li>
                <li>
                  Receives the same{" "}
                  <code className="rounded bg-muted px-1 py-0.5">
                    program_id
                  </code>{" "}
                  and{" "}
                  <code className="rounded bg-muted px-1 py-0.5">
                    aggregated_key
                  </code>
                </li>
              </ul>
            </div>
          </div>
          <div>
            <h4 className="font-medium">BitVMX Interactions:</h4>
            <ul className="mt-2 list-disc space-y-2 pl-6">
              <li>Creates aggregated key from participant public keys</li>
              <li>
                Validates that participant information matches between players
              </li>
              <li>Generates program ID from aggregated ID</li>
            </ul>
          </div>

          <div className="mt-6 overflow-hidden rounded-lg border bg-background">
            <img
              src="/game-flow/game-flow-1.png"
              alt="Setup Participants sequence diagram"
              className="w-full"
            />
          </div>
        </article>

        <article className="mt-10 space-y-4">
          <h3 className="text-xl font-semibold">Step 2: Place Bet</h3>
          <p className="text-muted-foreground">
            <strong>What happens:</strong> Player 1 places their bet by sending
            funds to the aggregated address.
          </p>
          <div className="grid gap-6 md:grid-cols-2">
            <div>
              <h4 className="font-medium">Player 1 Actions:</h4>
              <ul className="mt-2 list-disc space-y-2 pl-6">
                <li>
                  Calls{" "}
                  <code className="rounded bg-muted px-1 py-0.5">
                    /place-bet
                  </code>{" "}
                  with bet amount
                </li>
                <li>
                  System sends funds to aggregated address (protocol fees + bet
                  amount)
                </li>
                <li>Waits for transaction confirmation</li>
                <li>
                  System automatically transitions to{" "}
                  <span className="rounded bg-muted px-1 py-0.5">
                    SetupFunding
                  </span>{" "}
                  state
                </li>
              </ul>
            </div>
            <div>
              <h4 className="font-medium">Player 2 Actions:</h4>
              <ul className="mt-2 list-disc space-y-2 pl-6">
                <li>No action required - waits for Player 1 to place bet</li>
              </ul>
            </div>
          </div>
          <div>
            <h4 className="font-medium">Bitcoin Interactions:</h4>
            <ul className="mt-2 list-disc space-y-2 pl-6">
              <li>Sends funds to aggregated address</li>
              <li>Waits for transaction confirmation</li>
              <li>Creates funding UTXOs</li>
            </ul>
          </div>

          <div className="mt-6 overflow-hidden rounded-lg border bg-background">
            <img
              src="/game-flow/game-flow-2.png"
              alt="Place Bet sequence diagram"
              className="w-full"
            />
          </div>
        </article>

        <article className="mt-10 space-y-4">
          <h3 className="text-xl font-semibold">Step 3: Setup Funding UTXO</h3>
          <p className="text-muted-foreground">
            <strong>What happens:</strong> Player 2 waits for Player 1 to share
            the bet UTXOs and stores them for later use in the setup game step.
          </p>
          <div className="grid gap-6 md:grid-cols-2">
            <div>
              <h4 className="font-medium">Player 2 Actions:</h4>
              <ul className="mt-2 list-disc space-y-2 pl-6">
                <li>
                  Calls{" "}
                  <code className="rounded bg-muted px-1 py-0.5">
                    /setup-funding-utxo
                  </code>{" "}
                  to receive and store Player 1's bet UTXOs
                </li>
                <li>
                  System validates and stores the UTXOs for use in game setup
                </li>
              </ul>
            </div>
            <div>
              <h4 className="font-medium">Player 1 Actions:</h4>
              <ul className="mt-2 list-disc space-y-2 pl-6">
                <li>
                  No action required - bet UTXOs already known from place-bet
                  step
                </li>
              </ul>
            </div>
          </div>
          <div>
            <h4 className="font-medium">BitVMX Interactions:</h4>
            <ul className="mt-2 list-disc space-y-2 pl-6">
              <li>Validates funding transaction</li>
              <li>Sets up dispute transactions for protocol and bet UTXOs</li>
            </ul>
          </div>

          <div className="mt-6 overflow-hidden rounded-lg border bg-background">
            <img
              src="/game-flow/game-flow-3.png"
              alt="Setup Funding UTXO sequence diagram"
              className="w-full"
            />
          </div>
        </article>

        <article className="mt-10 space-y-4">
          <h3 className="text-xl font-semibold">Step 4: Setup Game</h3>
          <p className="text-muted-foreground">
            <strong>What happens:</strong> Both players configure the game with
            the numbers to sum. Both players must set up the same numbers,
            otherwise the game creation will fail. This ensures that all
            participants agree on the same inputs for the BitVMX program, as the
            game is implemented as a program in BitVMX.
          </p>
          <div>
            <h4 className="font-medium">Both Players Actions:</h4>
            <ul className="mt-2 list-disc space-y-2 pl-6">
              <li>
                Call{" "}
                <code className="rounded bg-muted px-1 py-0.5">
                  /setup-game
                </code>{" "}
                with the two numbers to sum
              </li>
              <li>Must use the same numbers as the other player</li>
              <li>System configures the BitVMX program with game parameters</li>
            </ul>
          </div>
          <div>
            <h4 className="font-medium">BitVMX Interactions:</h4>
            <ul className="mt-2 list-disc space-y-2 pl-6">
              <li>Sets program input with concatenated numbers</li>
              <li>Sets aggregated key variable</li>
              <li>Sets protocol and bet UTXOs</li>
              <li>Sets program definition file path</li>
              <li>Sets timelock blocks</li>
              <li>Performs program setup</li>
            </ul>
          </div>

          <div className="mt-6 overflow-hidden rounded-lg border bg-background">
            <img
              src="/game-flow/game-flow-4.png"
              alt="Setup Game sequence diagram"
              className="w-full"
            />
          </div>
        </article>

        <article className="mt-10 space-y-4">
          <h3 className="text-xl font-semibold">Step 5: Start Game</h3>
          <p className="text-muted-foreground">
            <strong>What happens:</strong> Player 1 initiates the game by
            sending a challenge transaction. START_CH (Start Challenge) begins
            the dispute process. From this point on, Player 1 becomes the
            Verifier and Player 2 becomes the Prover.
          </p>
          <div className="grid gap-6 md:grid-cols-2">
            <div>
              <h4 className="font-medium">Player 1 Actions (Verifier):</h4>
              <ul className="mt-2 list-disc space-y-2 pl-6">
                <li>
                  Calls{" "}
                  <code className="rounded bg-muted px-1 py-0.5">
                    /start-game
                  </code>
                </li>
                <li>System sends challenge transaction to start the game</li>
                <li>Enqueues job to wait for game outcome</li>
              </ul>
            </div>
            <div>
              <h4 className="font-medium">Player 2 Actions (Prover):</h4>
              <ul className="mt-2 list-disc space-y-2 pl-6">
                <li>
                  System automatically waits for the challenge transaction
                </li>
                <li>Enqueues job to wait for start game transaction</li>
              </ul>
            </div>
          </div>

          <div className="mt-6 overflow-hidden rounded-lg border bg-background">
            <img
              src="/game-flow/game-flow-5.png"
              alt="Start Game sequence diagram"
              className="w-full"
            />
          </div>
        </article>

        <article className="mt-10 space-y-4">
          <h3 className="text-xl font-semibold">Step 6: Submit Sum</h3>
          <p className="text-muted-foreground">
            <strong>What happens:</strong> Player 2 submits their guess and the
            dispute resolution process begins.
          </p>
          <div>
            <h4 className="font-medium">Player 2 Actions:</h4>
            <ul className="mt-2 list-disc space-y-2 pl-6">
              <li>
                Calls{" "}
                <code className="rounded bg-muted px-1 py-0.5">
                  /submit-sum
                </code>{" "}
                with their guess
              </li>
              <li>System sets the guess as program input</li>
              <li>Sends challenge input transaction</li>
            </ul>
          </div>
          <div>
            <h4 className="font-medium">BitVMX Interactions:</h4>
            <ul className="mt-2 list-disc space-y-2 pl-6">
              <li>Sets program input with guess</li>
              <li>Dispatches challenge input transaction</li>
              <li>
                Waits for dispute transactions to be confirmed: COMMITMENT,
                NARY_PROVER_1, NARY_VERIFIER_1, NARY_PROVER_2, NARY_VERIFIER_2,
                EXECUTE, PROVER_WINS_START, PROVER_WINS_SUCCESS,
                ACTION_PROVER_WINS
              </li>
              <li>Determines game outcome based on dispute results</li>
            </ul>
          </div>
          <div>
            <h4 className="font-medium">Bitcoin Interactions:</h4>
            <ul className="mt-2 list-disc space-y-2 pl-6">
              <li>
                Challenge input transaction is broadcasted to the Bitcoin
                network
              </li>
              <li>
                All dispute transactions are broadcasted to the Bitcoin network
              </li>
            </ul>
          </div>

          <div className="mt-6 overflow-hidden rounded-lg border bg-background">
            <img
              src="/game-flow/game-flow-6.png"
              alt="Submit Sum sequence diagram"
              className="w-full"
            />
          </div>
        </article>

        <article className="mt-10 space-y-4">
          <h3 className="text-xl font-semibold">Step 7: Game Completion</h3>
          <p className="text-muted-foreground">
            <strong>What happens:</strong> The game concludes with automatic
            winner determination.
          </p>
          <div className="grid gap-6 md:grid-cols-2">
            <div>
              <h4 className="font-medium">Both Players:</h4>
              <ul className="mt-2 list-disc space-y-2 pl-6">
                <li>Receive final game state with outcome</li>
              </ul>
            </div>
            <div>
              <h4 className="font-medium">BitVMX Interactions:</h4>
              <ul className="mt-2 list-disc space-y-2 pl-6">
                <li>Executes dispute resolution protocol</li>
                <li>Determines winner based on dispute results</li>
                <li>
                  Updates game state to{" "}
                  <span className="rounded bg-muted px-1 py-0.5">
                    GameComplete
                  </span>
                </li>
              </ul>
            </div>
          </div>

          <div className="mt-6 overflow-hidden rounded-lg border bg-background">
            <img
              src="/game-flow/game-flow-7.png"
              alt="Game Completion sequence diagram"
              className="w-full"
            />
          </div>
        </article>
      </section>

      <section className="mt-12">
        <h2 className="text-2xl font-semibold">
          Key BitVMX Interactions Summary
        </h2>
        <p className="mt-3 text-muted-foreground">
          Throughout the game, the following BitVMX messages are sent:
        </p>
        <div className="mt-4 grid gap-6 md:grid-cols-2">
          <div className="rounded-lg border p-4">
            <h4 className="font-medium">1. Key Management</h4>
            <ul className="mt-2 list-disc space-y-1 pl-6">
              <li>
                <code className="rounded bg-muted px-1 py-0.5">SetupKey</code> -
                Creates shared public key from participant information
              </li>
              <li>
                <code className="rounded bg-muted px-1 py-0.5">GetPubKey</code>{" "}
                - Gets operator public key
              </li>
              <li>
                <code className="rounded bg-muted px-1 py-0.5">
                  GetAggregatedPubkey
                </code>{" "}
                - Retrieves aggregated public key
              </li>
            </ul>
          </div>
          <div className="rounded-lg border p-4">
            <h4 className="font-medium">2. Fund Management</h4>
            <ul className="mt-2 list-disc space-y-1 pl-6">
              <li>
                <code className="rounded bg-muted px-1 py-0.5">SendFunds</code>{" "}
                - Sends funds to aggregated address
              </li>
              <li>
                <code className="rounded bg-muted px-1 py-0.5">
                  GetFundingAddress
                </code>{" "}
                - Gets funding address
              </li>
              <li>
                <code className="rounded bg-muted px-1 py-0.5">
                  GetFundingBalance
                </code>{" "}
                - Gets funding balance
              </li>
              <li>
                <code className="rounded bg-muted px-1 py-0.5">
                  GetTransaction
                </code>{" "}
                - Gets transaction status
              </li>
            </ul>
          </div>
          <div className="rounded-lg border p-4">
            <h4 className="font-medium">3. Program Configuration</h4>
            <ul className="mt-2 list-disc space-y-1 pl-6">
              <li>
                <code className="rounded bg-muted px-1 py-0.5">SetVar</code> -
                Sets program variables (aggregated key, UTXOs, etc.)
              </li>
              <li>
                <code className="rounded bg-muted px-1 py-0.5">Setup</code> -
                Initializes the BitVMX program with participants
              </li>
              <li>
                <code className="rounded bg-muted px-1 py-0.5">
                  SetFundingUtxo
                </code>{" "}
                - Sets funding UTXO for speed-up transactions
              </li>
            </ul>
          </div>
          <div className="rounded-lg border p-4">
            <h4 className="font-medium">4. Transaction Management</h4>
            <ul className="mt-2 list-disc space-y-1 pl-6">
              <li>
                <code className="rounded bg-muted px-1 py-0.5">
                  DispatchTransactionName
                </code>{" "}
                - Dispatches specific transactions (START_CH, challenge input,
                etc.)
              </li>
            </ul>
          </div>
          <div className="rounded-lg border p-4">
            <h4 className="font-medium">5. P2P Communication</h4>
            <ul className="mt-2 list-disc space-y-1 pl-6">
              <li>
                <code className="rounded bg-muted px-1 py-0.5">
                  GetCommInfo
                </code>{" "}
                - Gets P2P communication information
              </li>
            </ul>
          </div>
          <div className="rounded-lg border p-4">
            <h4 className="font-medium">6. Dispute Resolution</h4>
            <ul className="mt-2 list-disc space-y-1 pl-6">
              <li>
                Waits for multiple dispute transactions to determine game
                outcome
              </li>
              <li>Determines winner based on dispute results</li>
            </ul>
          </div>
        </div>

        <div className="mt-6 overflow-hidden rounded-lg border bg-background p-2">
          <img
            src="/game-flow/game-flow-8.svg"
            alt="Protocol graph"
            className="w-full"
          />
        </div>
      </section>

      <section className="mt-12">
        <h2 className="text-2xl font-semibold">Game Outcome Logic</h2>
        <ul className="mt-4 list-disc space-y-2 pl-6">
          <li>
            <strong>Player 2 Wins:</strong> If Player 2's guess is correct, all
            dispute transactions are confirmed, and Player 2 is determined as
            the winner
          </li>
          <li>
            <strong>Player 1 Wins:</strong> If Player 2's guess is incorrect,
            the dispute process times out, and Player 1 is determined as the
            winner
          </li>
        </ul>
        <p className="mt-3 text-muted-foreground">
          The game ensures fairness through BitVMX's cryptographic dispute
          resolution protocol, making it impossible for either player to cheat.
        </p>
      </section>
    </div>
  );
}
