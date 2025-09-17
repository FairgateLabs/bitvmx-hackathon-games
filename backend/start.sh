#!/bin/bash
set -e

pids=()

cleanup() {
    echo "⚠️ Cleaning up processes with SIGKILL..."
    
    # Kill tracked background processes
    for pid in "${pids[@]}"; do
        if kill -0 "$pid" 2>/dev/null; then
            echo "🔴 Sending SIGKILL to tracked process $pid"
            kill -9 "$pid" 2>/dev/null || true
        fi
    done
    
    # Also kill any remaining background jobs from this shell
    echo "🔴 Killing all background jobs..."
    jobs -p | xargs -r kill -9 2>/dev/null || true
    
    # Kill any child processes of this script
    echo "🔴 Killing child processes..."
    pkill -P $$ 2>/dev/null || true
}

# Function to handle Ctrl+C and propagate it to child processes
handle_interrupt() {
    echo "🛑 Interrupt received, stopping all processes..."
    cleanup
    exit 130  # Standard exit code for Ctrl+C
}

# Set up signal handling
trap handle_interrupt INT TERM
echo "🔧 Signal handlers set up for INT and TERM"

bash scripts/start-bitcoin.sh

# Wait for the container to start
echo "⏳ Waiting 5 second for bitcoind to start..."
sleep 5

echo "Starting auto mine..."
bash scripts/start-bitcoin-mine.sh & pids+=($!)
echo "Auto mine started with PID: $!"

echo "Starting bitvmx 1..."
bash scripts/start-op-1.sh & pids+=($!)
echo "BitVMX 1 started with PID: $!"
sleep 1
echo "Starting bitvmx 2..."
bash scripts/start-op-2.sh & pids+=($!)
echo "BitVMX 2 started with PID: $!"
sleep 1

# Wait a bit before launching the players
echo "⏳ Waiting 10 second for bitvmx to synchronize..."
sleep 5

bash scripts/start-player-1.sh & pids+=($!)
echo "Player 1 started with PID: $!"
sleep 1

bash scripts/start-player-2.sh & pids+=($!)
echo "Player 2 started with PID: $!"
sleep 1

# Wait for all, abort if any fails
echo "⏳ Waiting for all processes to complete..."
for pid in "${pids[@]}"; do
    echo "🔍 Waiting for process $pid..."
    wait "$pid" || {
        echo "⚠️ Process $pid exited with error, cleaning up..."
        cleanup
        exit 1
    }
done
echo "✅ All processes completed successfully"