#!/bin/bash
set -e

# we go to the root of the project to avoid relative path issues
CURRENT_PATH=$( cd "$(dirname "${BASH_SOURCE[0]}")/../" ; pwd -P )
cd "$CURRENT_PATH/";

LOG_PATH="$CURRENT_PATH/logs/player_1"

# go to the bitvmx client folder
BITVMX_PATH="$CURRENT_PATH/../deps/rust-bitvmx-client"

# get the config file information
CONFIG_FILE="op_1"
CONFIG_PATH="$BITVMX_PATH/config/$CONFIG_FILE.yaml"

# Check if config file exists
if [ ! -f "$CONFIG_PATH" ]; then
    echo "Error: Config file not found at $CONFIG_PATH"
    exit 1
fi

BROKER_PORT=$(grep -A 10 "^broker:" "$CONFIG_PATH" | grep "port:" | awk '{print $2}' | tr -d ' ')
STORAGE_PATH="temp-runs/$CONFIG_FILE.db"

bash scripts/run-bitvmx-dispatcher.sh -i 127.0.0.1 -p $BROKER_PORT -l $LOG_PATH -s $STORAGE_PATH