# Start auto mine 1 block per 5 seconds
BITCOIND_NAME=bitcoin-regtest
BLOCKS_TIME=2
docker exec $BITCOIND_NAME sh -c \
'bitcoin-cli -regtest -rpcuser=foo -rpcpassword=rpcpassword createwallet "default" && ADDRESS=$(bitcoin-cli -regtest -rpcuser=foo -rpcpassword=rpcpassword getnewaddress) && while true; do bitcoin-cli -regtest -rpcuser=foo -rpcpassword=rpcpassword generatetoaddress 1 $ADDRESS && echo "⛏️  Block mined" && sleep '$BLOCKS_TIME'; done'