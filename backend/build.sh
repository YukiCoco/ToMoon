#!/bin/bash

set -e
rustup default stable
cross build --release
mkdir -p ../bin
cp ./target/x86_64-unknown-linux-gnu/release/tomoon ../bin/tomoon
