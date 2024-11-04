#!/bin/bash
mkdir data > /dev/null 2>&1 &

# relaychain

nohup ./polkadot --alice -d data/node1 --chain config/rococo-local-raw.json --validator  --ws-port 9944 --rpc-port 10025 --port 30033  --rpc-cors all  -lapproval_voting=trace,sync=debug,staking=trace,babe=trace --pruning archive  > data/log.alice 2>&1 &
nohup ./polkadot --bob   -d data/node2 --chain config/rococo-local-raw.json --validator  --ws-port 9955 --rpc-port 10026 --port 30034  --rpc-cors all -lapproval_voting=trace > data/log.bob 2>&1 &
nohup ./polkadot --charlie   -d data/node3 --chain config/rococo-local-raw.json --validator  --ws-port 9966 --rpc-port 10027 --port 30035  --rpc-cors all -lapproval_voting=trace > data/log.charlie 2>&1 &
nohup ./polkadot --dave   -d data/node4 --chain config/rococo-local-raw.json --validator  --ws-port 9977 --rpc-port 10028 --port 30036  --rpc-cors all -lapproval_voting=trace > data/log.dave 2>&1 &

# parachain
nohup ./nbchain-node -d ./data/nbchain-1000 --alice --force-authoring --collator --discover-local --rpc-cors=all --ws-port 9988 --rpc-port 8545 --port 40041 --chain ./config/nbchain-dev-1000.json -llog=info -lruntime=debug,evm=trace --  --chain ./config/rococo-local-raw.json --discover-local --port 40042 > data/log.1000 2>&1 &
nohup ./nbchain-node -d ./data/nbchain-2000 --alice --force-authoring --collator --discover-local --rpc-cors=all --ws-port 9999 --rpc-port 8546 --port 40042 --chain ./config/nbchain-dev-2000.json -llog=info -lruntime=debug,evm=trace --  --chain ./config/rococo-local-raw.json --discover-local --port 40043 > data/log.2000 2>&1 &
