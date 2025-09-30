#!/bin/bash

# BitVMX Dependencies Install Script
# This script clones the required BitVMX repositories for the hackathon project

set -e  # Exit on any error

echo "🚀 Installing BitVMX Dependencies..."

# Create dependencies directory (relative to backend/)
DEPENDENCIES_DIR="dependencies"
mkdir -p "$DEPENDENCIES_DIR"

# Change to dependencies directory
cd "$DEPENDENCIES_DIR"

echo "📁 Created dependencies directory: $DEPENDENCIES_DIR"

# Clone repositories
echo "📥 Cloning BitVMX repositories..."

# BitVMX Client
echo "  - Cloning bitvmx-client..."
if [ ! -d "rust-bitvmx-client" ]; then
    git clone https://github.com/FairgateLabs/rust-bitvmx-client.git
    echo "    ✅ bitvmx-client cloned successfully"
else
    echo "    ⚠️  bitvmx-client already exists, skipping..."
fi

# BitVMX Broker
echo "  - Cloning bitvmx-broker..."
if [ ! -d "rust-bitvmx-broker" ]; then
    git clone https://github.com/FairgateLabs/rust-bitvmx-broker.git
    echo "    ✅ bitvmx-broker cloned successfully"
else
    echo "    ⚠️  bitvmx-broker already exists, skipping..."
fi

# BitVMX Bitcoin RPC
echo "  - Cloning bitvmx-bitcoin-rpc..."
if [ ! -d "rust-bitvmx-bitcoin-rpc" ]; then
    git clone https://github.com/FairgateLabs/rust-bitvmx-bitcoin-rpc.git
    echo "    ✅ bitvmx-bitcoin-rpc cloned successfully"
else
    echo "    ⚠️  bitvmx-bitcoin-rpc already exists, skipping..."
fi
# BitVMX Job Dispatcher
echo "  - Cloning bitvmx-job-dispatcher..."
if [ ! -d "rust-bitvmx-job-dispatcher" ]; then
    git clone https://github.com/FairgateLabs/rust-bitvmx-job-dispatcher.git
    echo "    ✅ bitvmx-job-dispatcher cloned successfully"
else
    echo "    ⚠️  bitvmx-job-dispatcher already exists, skipping..."
fi

# BitVMX CPU
echo "  - Cloning bitvmx-cpu..."
if [ ! -d "BitVMX-CPU" ]; then
    git clone https://github.com/FairgateLabs/BitVMX-CPU.git
    echo "    ✅ bitvmx-cpu cloned successfully"
else
    echo "    ⚠️  bitvmx-cpu already exists, skipping..."
fi


# Go back to backend directory
cd ..

echo ""
echo "🎉 All BitVMX dependencies have been installed!"
echo ""
echo "📂 Dependencies are located in: ./$DEPENDENCIES_DIR/"
echo "   - rust-bitvmx-client"
echo "   - rust-bitvmx-broker"
echo "   - rust-bitvmx-bitcoin-rpc"
echo ""

# Build the backend
echo "🔨 Building backend using build.sh script..."
if [ -f "scripts/build.sh" ]; then
    bash scripts/build.sh
    if [ $? -eq 0 ]; then
        echo "✅ Backend built successfully!"
    else
        echo "❌ Backend build failed. Please check the errors above."
        exit 1
    fi
else
    echo "⚠️  build.sh script not found. Please ensure the script exists in the scripts directory."
fi

echo ""
echo "💡 Next steps:"
echo "   1. Run backend: bash start.sh"
echo "   2. Run frontend: cd ../frontend && yarn dev:3000 and yarn dev:3001"
echo ""
