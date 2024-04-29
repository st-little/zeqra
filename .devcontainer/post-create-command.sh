#!/bin/sh

echo "Start postCreateCommand." `date '+%y/%m/%d %H:%M:%S'`

# Dioxus CLI
cargo install dioxus-cli@0.5.1

echo "Completed postCreateCommand." `date '+%y/%m/%d %H:%M:%S'`