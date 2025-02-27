#!/bin/bash

targets=("x86_64-unknown-linux-gnu", "x86_64-pc-windows-gnu")

for target in "${targets[@]}"; do
    echo "Compiling for $target with cross..."
    cross build --target "$target" --release
done