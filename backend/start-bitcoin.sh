#!/bin/bash
set -e

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

sleep 1

docker exec -it $CONTAINER_NAME sh -c \
'bitcoin-cli -regtest -rpcuser=foo -rpcpassword=rpcpassword createwallet "default" && ADDRESS=$(bitcoin-cli -regtest -rpcuser=foo -rpcpassword=rpcpassword getnewaddress) && while true; do bitcoin-cli -regtest -rpcuser=foo -rpcpassword=rpcpassword generatetoaddress 1 $ADDRESS && echo "⛏️  Bloque minado" && sleep 1; done'