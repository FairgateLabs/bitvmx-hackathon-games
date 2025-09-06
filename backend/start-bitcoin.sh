#!/bin/bash
set -e

# stop and remove the bitcoin-regtest container if it exists
BITCOIND_NAME=bitcoin-regtest
if [ -n "$(docker ps -a -f name="^$BITCOIND_NAME$" -q)" ]; then
    echo "Stopping and remove $BITCOIND_NAME container"
    docker rm -f $BITCOIND_NAME
fi

# stop and remove the bitcoin-regtest container if it exists
EXPLORER_NAME=btc-rpc-explorer
if [ -n "$(docker ps -a -f name="^$EXPLORER_NAME$" -q)" ]; then
    echo "Stopping and remove $EXPLORER_NAME container"
    docker rm -f $EXPLORER_NAME
fi

# Create a network for the containers
if ! docker network inspect bitcoin-net >/dev/null 2>&1; then
  docker network create bitcoin-net
fi

# Start the Bitcoin Regtest node https://hub.docker.com/r/ruimarinho/bitcoin-core/dockerfile
docker run --name $BITCOIND_NAME -d \
    --network bitcoin-net \
    -p 18443:18443 \
    -e BITCOIN_DATA=/data \
    ruimarinho/bitcoin-core \
        -regtest=1 \
        -printtoconsole \
        -rpcallowip=0.0.0.0/0 \
        -rpcbind=0.0.0.0 \
        -rpcuser=foo \
        -rpcpassword=rpcpassword \
        -server=1 \
        -txindex=1 \
        -fallbackfee=0.0002

# Wait for the container to start
sleep 2


# Start the Bitcoin RPC Explorer https://hub.docker.com/r/tyzbit/btc-rpc-explorer
docker run  --name $EXPLORER_NAME -d \
    --network bitcoin-net \
    -p 4000:4000 \
    --platform linux/amd64 \
    -e BTCEXP_PORT=4000 \
    -e BTCEXP_HOST=0.0.0.0  \
    -e BTCEXP_BITCOIND_HOST=$BITCOIND_NAME \
    -e BTCEXP_BITCOIND_PORT=18443 \
    -e BTCEXP_BITCOIND_USER=foo \
    -e BTCEXP_BITCOIND_PASS=rpcpassword \
    tyzbit/btc-rpc-explorer

# Wait for the container to start
sleep 1

# Start auto mine blocks one per second
docker exec -it $BITCOIND_NAME sh -c \
'bitcoin-cli -regtest -rpcuser=foo -rpcpassword=rpcpassword createwallet "default" && ADDRESS=$(bitcoin-cli -regtest -rpcuser=foo -rpcpassword=rpcpassword getnewaddress) && while true; do bitcoin-cli -regtest -rpcuser=foo -rpcpassword=rpcpassword generatetoaddress 1 $ADDRESS && echo "⛏️  Bloque minado" && sleep 1; done'