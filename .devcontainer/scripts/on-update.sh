#!/usr/bin/env bash

# This script is when the container is updated.

set -e

# Make sure mise is activated
eval "$(mise activate bash --shims)"

# Ensure mise is activated
PATH="$HOME/.local/bin:$PATH"
eval "$(mise activate bash --shims)"

# Pull git submodules
if [ -d .git ]; then
  git submodule update --recursive --init --remote
fi

# Trust all mise configs
mise trust --yes --all
if [ -d .git ]; then
  git submodule foreach --recursive "mise trust"
fi

# Update mise
mise self-update -y

# Install stack
mise install

# Install dependencies

# Rust
if [ -f ./Cargo.lock ]; then
  cargo build || echo "🟡 Cargo build failed, but that's ok"
fi

# # Node.js
if [ -f ./pnpm-lock.yaml ]; then
  yes | pnpm install
elif [ -f ./yarn.lock ]; then
  yes | yarn install
elif [ -f ./package-lock.json ]; then
  yes | npm install
fi