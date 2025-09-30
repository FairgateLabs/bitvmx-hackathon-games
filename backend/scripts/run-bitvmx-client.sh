#!/bin/bash
set -e

# Global variables
CONFIG_FILE=""
LOG_PATH=""

# Parse command line arguments using getopts
while getopts "c:l:h" opt; do
    case $opt in
        c)
            CONFIG_FILE="$OPTARG"
            ;;
        l)
            LOG_PATH="$OPTARG"
            ;;
        h)
            usage
            ;;
        \?)
            echo "Invalid option: -$OPTARG" >&2
            usage
            ;;
        :)
            echo "Option -$OPTARG requires an argument." >&2
            usage
            ;;
    esac
done

# Check if required parameters are provided
if [ -z "$CONFIG_FILE" ]; then
    echo "Error: CONFIG_FILE parameter is required"
    echo "Use -c to specify the config file"
    usage
fi

if [ -z "$LOG_PATH" ]; then
    echo "Error: LOG_PATH parameter is required"
    echo "Use -l to specify the log path"
    usage
fi

echo "Starting bitvmx-client..."
echo "CONFIG_FILE: $CONFIG_FILE"
echo "LOG_PATH: $LOG_PATH"

# we go to the root of the project to avoid relative path issues
CURRENT_PATH=$( cd "$(dirname "${BASH_SOURCE[0]}")/../../" ; pwd -P )
cd "$CURRENT_PATH"

echo "CURRENT_PATH: $CURRENT_PATH"

# go to the bitvmx client folder
BITVMX_PATH="$CURRENT_PATH/deps/rust-bitvmx-client"

echo "BITVMX_PATH: $BITVMX_PATH"

cd "$BITVMX_PATH"

# get the config file information
CONFIG_PATH="$BITVMX_PATH/config/$CONFIG_FILE.yaml"
P2P_PORT=$(grep "address: /ip4/127.0.0.1/tcp/" "$CONFIG_PATH" | awk '{print $2}' | sed 's|/ip4/127.0.0.1/tcp/||')

# remove bitvmx client tmp data
rm -rf /tmp/broker_p2p_$P2P_PORT
rm -rf "/tmp/$CONFIG_FILE"
rm -rf "/tmp/regtest/$CONFIG_FILE"

# create logs directory if it doesn't exist
mkdir -p "$LOG_PATH"

# remove the log file
rm -rf "$LOG_PATH/bitvmx.log"

# run the bitvmx client with output to both console and file
RUST_LOG="debug,bitvmx_wallet::wallet=off,bitvmx_bitcoin_rpc=off,bitcoincore_rpc=off,hyper_util=off,libp2p=off,bitvmx_transaction_monitor=off,bitcoin_indexer=off,bitcoin_coordinator=off,p2p_protocol=off,p2p_handler=off,tarpc=off,broker=off" \
RUST_BACKTRACE=1 "$BITVMX_PATH/target/release/bitvmx-client" $CONFIG_FILE 2>&1 | while IFS= read -r line; do echo "$line"; echo "$line" | sed -r 's/\x1B\[([0-9]{1,2}(;[0-9]{1,2})*)?[mGKHF]//g' >> "$LOG_PATH/bitvmx.log"; done

# =============================================================================
# FUNCTIONS
# =============================================================================

# Function to display usage
usage() {
    echo "Usage: $0 -c CONFIG_FILE -l LOG_PATH"
    echo "  -c CONFIG_FILE  The config file to use (e.g., op_1, op_2)"
    echo "  -l LOG_PATH     The path where logs should be stored"
    echo ""
    echo "Example: $0 -c op_1 -l /path/to/logs"
    exit 1
}
