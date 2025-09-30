#!/bin/bash
set -e

# Set the current path to the root of the project
CURRENT_PATH=$( cd "$(dirname "${BASH_SOURCE[0]}")/.." ; pwd -P )
cd "$CURRENT_PATH"

# build the backend
cargo build

# build the job dispatcher
cd "$CURRENT_PATH/dependencies/rust-bitvmx-job-dispatcher"
cargo build --release

# build the BitVMX CPU
cd "$CURRENT_PATH/dependencies/BitVMX-CPU"
cargo build --release

# build the bitvmx client
cd "$CURRENT_PATH/dependencies/rust-bitvmx-client"
cargo build --release