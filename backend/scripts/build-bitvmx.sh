#!/bin/bash
set -e

# we go to the root of the project to avoid relative path issues
CURRENT_PATH=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
cd "$CURRENT_PATH/../";

# go to the bitvmx workspace
BITVMX_PATH="$CURRENT_PATH/../../../rust-bitvmx-workspace"

# build the job dispatcher
cd "$BITVMX_PATH/rust-bitvmx-job-dispatcher"
cargo build --release

# build the BitVMX CPU
cd "$BITVMX_PATH/BitVMX-CPU"
cargo build --release

# build the bitvmx client
cd "$BITVMX_PATH/rust-bitvmx-client"
cargo build --release