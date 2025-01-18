#!/bin/sh

echo "Start postCreateCommand." `date '+%y/%m/%d %H:%M:%S'`

sudo apt-get update && sudo apt-get install -y libssl-dev pkg-config

# Dioxus CLI
cargo install dioxus-cli@0.6.1

echo "Completed postCreateCommand." `date '+%y/%m/%d %H:%M:%S'`