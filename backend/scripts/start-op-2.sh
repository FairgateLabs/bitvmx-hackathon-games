#!/bin/bash
set -e

# we go to the root of the project to avoid relative path issues
CURRENT_PATH=$( cd "$(dirname "${BASH_SOURCE[0]}")/../" ; pwd -P )
cd "$CURRENT_PATH";

# logs directory
LOG_PATH="$CURRENT_PATH/logs/player_2"

bash scripts/run-bitvmx-client.sh -c op_2 -l $LOG_PATH