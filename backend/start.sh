#!/bin/bash
set -e

pids=()

bash start-bitcoin.sh & pids+=($!)

# Wait a bit before launching bitvmx
echo "⏳ Waiting 1 second for bitcoind to start..."
sleep 1

bash start-op-1.sh & pids+=($!)
bash start-op-2.sh & pids+=($!)

# Wait a bit before launching the players
echo "⏳ Waiting 1 second for bitvmx to start..."
sleep 1

bash start-player-1.sh & pids+=($!)
bash start-player-2.sh & pids+=($!)

# Wait for all and abort if any fails
for pid in "${pids[@]}"; do
    if ! wait "$pid"; then
        echo "❌ One of the scripts failed"
        exit 1
    fi
done