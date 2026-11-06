#!/bin/sh
set -eu

sudo chown node:node \
    .build \
    scripts/.venv \
    third_party/.cache \
    /home/node/.cache \
    /home/node/.cargo/registry \
    /home/node/.cargo/git

just install

scripts/.venv/bin/python -B build/lib/livekit.py "$(rustc -vV | awk '$1 == "host:" { print $2 }')"
