#!/bin/bash

pkill polkadot
pkill nbchain-node

rm -fr config
rm -fr data

rm ./nbchain-node
