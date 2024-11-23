#!/bin/sh

if ! aoc --version >/dev/null 2>&1; then
    cargo install aoc-cli
fi

set -xe

DAY=${1:-$(date +%d)}

aoc download -o -y 2024 -d $DAY -i inputs/input-$DAY.txt -p inputs/puzzle-$DAY.md
