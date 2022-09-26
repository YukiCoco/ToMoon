#!/bin/bash

cross build --release
cp ./target/x86_64-unknown-linux-gnu/release/clashdeck-rs ../bin/backend
