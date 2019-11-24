#! /usr/bin/env bash

cargo fmt
CHANGED=$(git status -s)
if [[ -z "$CHANGED" ]]; then
    echo "Working tree is dirty after formatting"
    exit 1
fi

cargo clippy -- -D warnings

echo "Formatting complete"
