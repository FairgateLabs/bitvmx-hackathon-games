#!/bin/bash
set -e

# we go to the root of the project to avoid relative path issues
CURRENT_PATH=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
cd "$CURRENT_PATH";

# stop and remove the bitcoin-regtest container if it exists
CONTAINER_NAME=bitcoin-regtest
if [ -n "$(docker ps -a -f name="^$CONTAINER_NAME$" -q)" ]; then
    echo "Stopping and remove $CONTAINER_NAME container"
    docker rm -f $CONTAINER_NAME
fi

# Start the Bitcoin Regtest node
docker run --name $CONTAINER_NAME -d -p 18443:18443 -e BITCOIN_DATA=/data ruimarinho/bitcoin-core \
        -regtest=1 \
        -printtoconsole \
        -rpcallowip=0.0.0.0/0 \
        -rpcbind=0.0.0.0 \
        -rpcuser=foo \
        -rpcpassword=rpcpassword \
        -server=1 \
        -txindex=1 \
        -fallbackfee=0.0002

# remove bitvmx client tmp data
rm -rf /tmp/broker_p2p_6118*
rm -rf /tmp/op_*

# go to the bitvmx client workspace
BITVMX_PATH="$CURRENT_PATH/../../rust-bitvmx-workspace/rust-bitvmx-client"
cd "$BITVMX_PATH"
# run the bitvmx client
RUST_LOG=debug cargo run op_1