#!/bin/bash
set -e

pids=()

# Set up signal handling
trap handle_interrupt INT TERM
echo "🔧 Signal handlers set up for INT and TERM"

# Build rust projects
bash scripts/build.sh

# Start bitcoin
bash scripts/start-bitcoin.sh

# Wait for the container to start
echo "⏳ Waiting 5 second for bitcoind to start..."
sleep 5

# Start bitvmx clients
echo "Starting bitvmx 1..."
bash scripts/start-op-1.sh & pids+=($!)
echo "BitVMX 1 started with PID: $!"
sleep 1

echo "Starting bitvmx 2..."
bash scripts/start-op-2.sh & pids+=($!)
echo "BitVMX 2 started with PID: $!"
sleep 1

# Wait a bit for bitvmx to synchronize
echo "⏳ Waiting 5 second for bitvmx to synchronize..."
sleep 5

# Start bitvmx dispatcher
echo "Starting bitvmx dispatcher 1..."
bash scripts/start-dispatcher-1.sh & pids+=($!)
echo "BitVMX dispatcher 1 started with PID: $!"
sleep 1

echo "Starting bitvmx dispatcher 2..."
bash scripts/start-dispatcher-2.sh & pids+=($!)
echo "BitVMX dispatcher 2 started with PID: $!"
sleep 1

# Start players backend
echo "Starting player 1..."
bash scripts/start-player-1.sh & pids+=($!)
echo "Player 1 started with PID: $!"
sleep 1

# Wait a bit before launching the players
echo "⏳ Waiting 1 second for player 1 to synchronize..."
sleep 1

echo "Starting player 2..."
bash scripts/start-player-2.sh & pids+=($!)
echo "Player 2 started with PID: $!"
sleep 1

# Wait a bit before launching the players
echo "⏳ Waiting 1 second for player 2 to synchronize..."
sleep 1

# Start auto mine
echo "Starting auto mine..."
bash scripts/start-auto-mine.sh & pids+=($!)
echo "Auto mine started with PID: $!"
sleep 1

# Wait a bit before launching the players
echo "⏳ Waiting 5 second for everyone to synchronize..."
sleep 5

# Show logs in real-time
echo "📋 Monitoring logs (press Ctrl+C to stop all processes)..."
echo "📁 Log files:"
echo "   - BitVMX 1: logs/op_1.log/bitvmx"
echo "   - BitVMX 2: logs/op_2.log/bitvmx" 
echo "   - Player 1: logs/player_1.log/backend"
echo "   - Player 2: logs/player_2.log/backend"
echo ""

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

# =============================================================================
# FUNCTIONS
# =============================================================================

# Function to handle Ctrl+C and propagate it to child processes
handle_interrupt() {
    echo "🛑 Interrupt received, stopping all processes..."
    cleanup
    exit 130  # Standard exit code for Ctrl+C
}

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