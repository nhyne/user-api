#! /usr/bin/env bash

cargo fmt
if [[ -z git status -s ]]; then
    echo "Working tree is dirty after formatting"
    exit 1
fi

cargo clippy -- -D warnings
