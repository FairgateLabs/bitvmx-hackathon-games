# Add Numbers Game Flow

This document explains the complete flow of the Add Numbers game, a BitVMX-based game where two players compete to solve a mathematical challenge. The game uses Bitcoin's dispute resolution protocol to ensure fair play and automatic fund distribution.

## Game Overview

The Add Numbers game is a two-player game where:

- **Player 1** creates the game and provides two numbers to sum
- **Player 2** joins the game and tries to guess the correct sum
- The game uses BitVMX's dispute resolution protocol to verify the answer
- The winner automatically receives the bet funds

## Game States

The game progresses through the following states:

1. `Setup Participants` - Initialize game participants and create aggregated key
2. `Place Bet` - Players place their bets
3. `Setup Funding` - Set up funding UTXOs for the game
4. `Setup Game` - Configure the game with numbers to sum
5. `Start Game` - Player 1 starts the challenge
6. `Submit Game Data` - Player 2 submits their guess
7. `Game Complete` - Game ends with winner determined
8. `Finished` - Final state

---

## Step-by-Step Game Flow

### Step 1: Setup Participants

**What happens:** Both players initialize the game by creating an aggregated key and sharing participant information.

**Player 1 Actions:**

- Calls `/setup-participants` with their P2P address, public key, and role as Player1
- Receives a `program_id` and `aggregated_key`

**Player 2 Actions:**

- Calls `/setup-participants` with the same `aggregated_id` and their information
- Receives the same `program_id` and `aggregated_key`

**BitVMX Interactions:**

- Creates aggregated key from participant public keys
- Generates program ID from aggregated ID

```mermaid
sequenceDiagram
    participant P1 as Player 1
    participant P2 as Player 2
    participant Game as Game Service
    participant BitVMX as BitVMX Service
    
    P1->>Game: POST /setup-participants
    Game->>BitVMX: create_agregated_key()
    BitVMX-->>Game: aggregated_key
    Game-->>P1: program_id, aggregated_key
    
    P2->>Game: POST /setup-participants (same aggregated_id)
    Game->>BitVMX: create_agregated_key()
    BitVMX-->>Game: aggregated_key
    Game-->>P2: program_id, aggregated_key
```

### Step 2: Place Bet

**What happens:** Players place their bets by sending funds to the aggregated address.

**Player 1 Actions:**

- Calls `/place-bet` with bet amount
- System sends funds to aggregated address (protocol fees + bet amount)
- Waits for transaction confirmation

**Player 2 Actions:**

- Calls `/place-bet` with bet amount
- System automatically transitions to `SetupFunding` state

**BitVMX Interactions:**

- Sends funds to aggregated address
- Waits for transaction confirmation
- Creates funding UTXOs

```mermaid
sequenceDiagram
    participant P1 as Player 1
    participant P2 as Player 2
    participant Game as Game Service
    participant BitVMX as BitVMX Service
    
    P1->>Game: POST /place-bet
    Game->>BitVMX: send_funds()
    BitVMX-->>Game: funding_txid
    Game->>BitVMX: wait_transaction_response()
    BitVMX-->>Game: tx_status
    Game-->>P1: game updated
    
    P2->>Game: POST /place-bet
    Game-->>P2: game (status: SetupFunding)
```

### Step 3: Setup Funding UTXO

**What happens:** Player 2 provides their funding UTXOs to complete the funding setup.

**Player 2 Actions:**

- Calls `/setup-funding-utxo` with their funding UTXOs
- System validates and stores the UTXOs

**BitVMX Interactions:**

- Validates funding transaction
- Sets up dispute transactions for protocol and bet UTXOs

```mermaid
sequenceDiagram
    participant P2 as Player 2
    participant Game as Game Service
    participant BitVMX as BitVMX Service
    
    P2->>Game: POST /setup-funding-utxo
    Game->>BitVMX: get_transaction()
    BitVMX-->>Game: tx_status
    Game->>Game: set_dispute_tx()
    Game-->>P2: success
```

### Step 4: Setup Game

**What happens:** Both players configure the game with the numbers to sum.

**Both Players Actions:**

- Call `/setup-game` with the two numbers to sum
- System configures the BitVMX program with game parameters

**BitVMX Interactions:**

- Sets program input with concatenated numbers
- Sets aggregated key variable
- Sets protocol and bet UTXOs
- Sets program definition file path
- Sets timelock blocks
- Performs program setup

```mermaid
sequenceDiagram
    participant P1 as Player 1
    participant P2 as Player 2
    participant Game as Game Service
    participant BitVMX as BitVMX Service
    
    P1->>Game: POST /setup-game (number1, number2)
    Game->>BitVMX: set_program_input()
    Game->>BitVMX: set_variable(aggregated)
    Game->>BitVMX: set_variable(utxo)
    Game->>BitVMX: set_variable(utxo_prover_win_action)
    Game->>BitVMX: set_variable(program_definition)
    Game->>BitVMX: set_variable(timelock_blocks)
    Game->>BitVMX: program_setup()
    BitVMX-->>Game: setup completed
    Game-->>P1: success
    
    P2->>Game: POST /setup-game (same numbers)
    Game->>BitVMX: [same setup process]
    Game-->>P2: success
```

### Step 5: Start Game

**What happens:** Player 1 initiates the game by sending a challenge transaction.

**Player 1 Actions:**

- Calls `/start-game`
- System sends challenge transaction to start the game
- Enqueues job to wait for game outcome

**Player 2 Actions:**

- System automatically waits for the challenge transaction
- Enqueues job to wait for start game transaction

**BitVMX Interactions:**

- Dispatches START_CH transaction
- Returns challenge transaction status

```mermaid
sequenceDiagram
    participant P1 as Player 1
    participant P2 as Player 2
    participant Game as Game Service
    participant BitVMX as BitVMX Service
    participant Worker as Worker Service
    
    P1->>Game: POST /start-game
    Game->>BitVMX: start_challenge()
    BitVMX-->>Game: challenge_tx
    Game->>Worker: handle_player2_wins_game_outcome_tx()
    Game-->>P1: challenge_tx
    
    Note over P2,Worker: Background job starts
    Worker->>BitVMX: wait_transaction_by_name_response(START_CH)
```

### Step 6: Submit Sum

**What happens:** Player 2 submits their guess and the dispute resolution process begins.

**Player 2 Actions:**

- Calls `/submit-sum` with their guess
- System sets the guess as program input
- Sends challenge input transaction

**BitVMX Interactions:**

- Sets program input with guess
- Dispatches challenge input transaction
- Waits for dispute transactions to be confirmed
- Determines game outcome based on dispute results

```mermaid
sequenceDiagram
    participant P2 as Player 2
    participant Game as Game Service
    participant BitVMX as BitVMX Service
    
    P2->>Game: POST /submit-sum (guess)
    Game->>BitVMX: set_program_input(guess)
    Game->>BitVMX: send_transaction_by_name()
    BitVMX-->>Game: challenge_input_tx
    
    Game->>BitVMX: wait_dispute_transactions()
    Note over BitVMX: Waits for multiple dispute txs:<br/>COMMITMENT, NARY_PROVER_1,<br/>NARY_VERIFIER_1, NARY_PROVER_2,<br/>NARY_VERIFIER_2, EXECUTE,<br/>PROVER_WINS_START, PROVER_WINS_SUCCESS,<br/>ACTION_PROVER_WINS
    
    alt Player 2 wins (correct guess)
        BitVMX-->>Game: dispute transactions confirmed
        Game-->>P2: game (outcome: Win)
    else Player 1 wins (incorrect guess)
        BitVMX-->>Game: timeout error
        Game-->>P2: game (outcome: Lose)
    end
```

### Step 7: Game Completion

**What happens:** The game concludes with automatic winner determination and fund distribution.

**Both Players:**

- Receive final game state with outcome
- Funds are automatically distributed to the winner through BitVMX protocol

**BitVMX Interactions:**

- Executes dispute resolution protocol
- Automatically transfers funds to winner
- Updates game state to `GameComplete`

```mermaid
sequenceDiagram
    participant P1 as Player 1
    participant P2 as Player 2
    participant Game as Game Service
    participant BitVMX as BitVMX Service
    
    Note over BitVMX: Dispute resolution completes
    
    alt Player 2 wins
        BitVMX->>BitVMX: transfer funds to Player 2
        Game->>Game: set_game_complete(Win, Challenge)
    else Player 1 wins
        BitVMX->>BitVMX: transfer funds to Player 1
        Game->>Game: set_game_complete(Lose, Challenge)
    end
    
    Game-->>P1: game (status: GameComplete)
    Game-->>P2: game (status: GameComplete)
```

---

## Key BitVMX Interactions Summary

Throughout the game, the following BitVMX operations are performed:

1. **Key Management:**
   - `create_agregated_key()` - Creates shared public key
   - `get_pub_key()` - Gets operator public key

2. **Fund Management:**
   - `send_funds()` - Sends funds to aggregated address
   - `wait_transaction_response()` - Waits for transaction confirmation
   - `get_transaction()` - Gets transaction status

3. **Program Configuration:**
   - `set_variable()` - Sets program variables (aggregated key, UTXOs, etc.)
   - `set_program_input()` - Sets input data (numbers, guess)
   - `program_setup()` - Initializes the BitVMX program

4. **Transaction Management:**
   - `start_challenge()` - Dispatches challenge transaction
   - `send_transaction_by_name()` - Sends specific transactions
   - `wait_transaction_by_name_response()` - Waits for specific transactions

5. **Dispute Resolution:**
   - Waits for multiple dispute transactions to determine game outcome
   - Automatically executes fund transfers based on dispute results

## Game Outcome Logic

- **Player 2 Wins:** If Player 2's guess is correct, all dispute transactions are confirmed, and Player 2 receives the bet funds
- **Player 1 Wins:** If Player 2's guess is incorrect, the dispute process times out, and Player 1 receives the bet funds

The game ensures fairness through BitVMX's cryptographic dispute resolution protocol, making it impossible for either player to cheat.
