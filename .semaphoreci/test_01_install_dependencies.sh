#!/bin/bash
set -Eeu

sudo apt-get update
sudo apt-get install -y python3 python3-requests
curl https://sh.rustup.rs -sSf | sh -s -- -y
