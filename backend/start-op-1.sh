#!/bin/bash
set -e

# we go to the root of the project to avoid relative path issues
CURRENT_PATH=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
cd "$CURRENT_PATH";

# remove bitvmx client tmp data
rm -rf /tmp/broker_p2p_61180
rm -rf /tmp/op_1

# create logs directory if it doesn't exist
mkdir -p logs/player_1.log

# remove the log file
rm -rf logs/player_1.log/bitvmx

# go to the bitvmx client workspace
BITVMX_PATH="$CURRENT_PATH/../../rust-bitvmx-workspace/rust-bitvmx-client"
cd "$BITVMX_PATH"

# set logs level for bitvmx client
RUST_LOG="debug,bitcoincore_rpc=off,hyper_util=off,libp2p=off,bitvmx_transaction_monitor=off,bitcoin_indexer=off,bitcoin_coordinator=info,p2p_protocol=off,p2p_handler=off,tarpc=off,broker=off"
 
# run the bitvmx client with output to both console and file
cargo run op_1 2>&1 | tee "$CURRENT_PATH/logs/player_1.log/bitvmx"