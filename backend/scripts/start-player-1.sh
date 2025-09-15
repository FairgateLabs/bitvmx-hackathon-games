#!/bin/bash
set -e

# we go to the root of the project to avoid relative path issues
CURRENT_PATH=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
cd "$CURRENT_PATH/../";

# remove the log file (no need to create the directory its done by the program)
rm -rf "$CURRENT_PATH/../logs/player_1.log/backend"

# run the backend
RUST_LOG=debug CONFIG_FILE=player_1 RUST_BACKTRACE=1 cargo run