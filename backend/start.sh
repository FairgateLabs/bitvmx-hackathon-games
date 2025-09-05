#!/bin/bash
set -e

pids=()

cleanup() {
    echo "⚠️ Cleaning up processes with SIGINT..."
    for pid in "${pids[@]}"; do
        if kill -0 "$pid" 2>/dev/null; then
            echo "🔴 Sending Ctrl+C (SIGINT) to process $pid"
            kill -s SIGINT "$pid" 2>/dev/null || true
        fi
    done
}
trap cleanup EXIT

bash start-bitcoin.sh > /dev/null & pids+=($!)

# Wait a bit before launching bitvmx
echo "⏳ Waiting 5 second for bitcoind to start..."
sleep 5

bash start-op-1.sh & pids+=($!)
bash start-op-2.sh & pids+=($!)

# Wait a bit before launching the players
echo "⏳ Waiting 5 second for bitvmx to start..."
sleep 5

bash start-player-1.sh & pids+=($!)
bash start-player-2.sh & pids+=($!)

# Wait for all, abort if any fails
for pid in "${pids[@]}"; do
    wait "$pid"
done