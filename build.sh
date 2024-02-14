#!/bin/bash

# Get name of current directory
REPO_NAME=$(basename "$(pwd)")

SAT_NAME=$REPO_NAME

# Replace '-service' with '' in REPO_NAME for GROUND_NAME
GROUND_NAME=$(echo "$REPO_NAME" | sed 's/-service//g')"-terminal"

# Build satellite image
cargo kubos -c build --target kubos-linux-isis-gcc -- --release
mv ./target/armv5te-unknown-linux-gnueabi/release/$REPO_NAME ./$SAT_NAME

# Build ground station image
cargo build --features terminal --release
mv ./target/release/$REPO_NAME ./$GROUND_NAME