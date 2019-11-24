#! /usr/bin/env bash

cargo fmt
git diff-files --quiet
CHANGED_RC=$?
if [[ "$CHANGED_RC" -eq 1 ]]; then
    echo "Working tree is dirty after formatting"
    exit 1
fi

cargo clippy -- -D warnings

echo "Formatting complete"
