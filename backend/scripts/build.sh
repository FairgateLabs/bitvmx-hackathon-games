#!/bin/bash
set -e

# Set the current path to the root of the project
CURRENT_PATH=$( cd "$(dirname "${BASH_SOURCE[0]}")/.." ; pwd -P )
cd "$CURRENT_PATH"

echo "🔨 Building the job dispatcher..."
# build the job dispatcher
cd "$CURRENT_PATH/dependencies/rust-bitvmx-job-dispatcher"
cargo build --release
echo "✅ Job dispatcher built successfully!"

echo "🔨 Building the BitVMX CPU..."
# build the BitVMX CPU
cd "$CURRENT_PATH/dependencies/BitVMX-CPU"
cargo build --release
echo "✅ BitVMX CPU built successfully!"

echo "🔨 Building the bitvmx client..."
# build the bitvmx client
cd "$CURRENT_PATH/dependencies/rust-bitvmx-client"
cargo build --release
echo "✅ BitVMX client built successfully!"

echo "🔨 Building the backend..."
# build the backend
cargo build
echo "✅ Backend built successfully!"
