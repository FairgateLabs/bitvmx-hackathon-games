#!/bin/bash
set -e

# we go to the root of the project to avoid relative path issues
CURRENT_PATH=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
cd "$CURRENT_PATH";

# run the bitvmx client
RUST_LOG=debug CONFIG_FILE=player_1 cargo run