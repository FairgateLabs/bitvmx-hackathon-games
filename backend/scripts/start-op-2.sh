#!/bin/bash
set -e

# we go to the root of the project to avoid relative path issues
CURRENT_PATH=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
cd "$CURRENT_PATH/../";

# remove bitvmx client tmp data
rm -rf /tmp/broker_p2p_61181
rm -rf /tmp/op_2
rm -rf /tmp/regtest/op_2

# create logs directory if it doesn't exist
LOG_PATH="$CURRENT_PATH/../logs/player_2"
mkdir -p "$LOG_PATH"

# remove the log file
rm -rf "$LOG_PATH/bitvmx.log"

# go to the bitvmx client workspace
BITVMX_PATH="$CURRENT_PATH/../../../rust-bitvmx-workspace/rust-bitvmx-client"
cd "$BITVMX_PATH"
 
# run the bitvmx client with output to both console and file
RUST_LOG="debug,bitvmx_wallet::wallet=off,bitvmx_bitcoin_rpc=off,bitcoincore_rpc=off,hyper_util=off,libp2p=off,bitvmx_transaction_monitor=off,bitcoin_indexer=off,bitcoin_coordinator=info,p2p_protocol=off,p2p_handler=off,tarpc=off,broker=off" \
script -q /dev/null bash -c "RUST_BACKTRACE=1 cargo run op_2 2>&1 | sed -r 's/\x1B\[([0-9]{1,2}(;[0-9]{1,2})*)?[mGKHF]//g'" | tee "$LOG_PATH/bitvmx.log"