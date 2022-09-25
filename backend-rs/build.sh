#!/bin/bash

cross build --release
mkdir ../bin
cp ./target/release/clashdeck-rs ../bin/backend
