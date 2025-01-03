#!/bin/bash

#
# Ferrous Systems Cloudflare Deployment Script
#
# Copyright (c) Ferrous Systems, 2024
#
# This script will find every tag in this repo and build the material.

set -euo pipefail

# We only support macOS (the x86 binaries work OK on Apple Silicon), or x86-64 Linux
if [ $(uname) == "Darwin" ]; then
    ./mdbook --version || curl -sSL https://github.com/rust-lang/mdBook/releases/download/v0.4.40/mdbook-v0.4.40-x86_64-apple-darwin.tar.gz | tar -xvzf -
    ./mdbook-mermaid --version || curl -sSL https://github.com/badboy/mdbook-mermaid/releases/download/v0.13.0/mdbook-mermaid-v0.13.0-x86_64-apple-darwin.tar.gz | tar -xvzf -
    ./mdslides --version || ( curl -sSL https://github.com/ferrous-systems/mdslides/releases/download/v0.4.0/mdslides-x86_64-apple-darwin.tar.xz | tar -xvJf - \
        && mv ./mdslides-*/mdslides . \
        && rm -rf ./mdslides-*/ )
else
    ./mdbook --version || curl -sSL https://github.com/rust-lang/mdBook/releases/download/v0.4.40/mdbook-v0.4.40-x86_64-unknown-linux-gnu.tar.gz | tar -xvzf -
    ./mdbook-mermaid --version || curl -sSL https://github.com/badboy/mdbook-mermaid/releases/download/v0.13.0/mdbook-mermaid-v0.13.0-x86_64-unknown-linux-gnu.tar.gz | tar -xvzf -
    ./mdslides --version || ( curl -sSL https://github.com/ferrous-systems/mdslides/releases/download/v0.4.0/mdslides-x86_64-unknown-linux-gnu.tar.xz | tar -xvJf - \
        && mv ./mdslides-*/mdslides . \
        && rm -rf ./mdslides-*/ )
fi

# Must be an absolute path, otherwise mdbook puts the output in the wrong place
OUTPUT_DIR=$(pwd)/html

# Clean out whatever exists, and make the index (before we do any git checkouts
# and lose the files from this checkout)
rm -rf "${OUTPUT_DIR}"
mkdir -p "${OUTPUT_DIR}"
cp ./_redirects "${OUTPUT_DIR}/_redirects"

# Build the book and slides
function build_and_store {
    mkdir -p "${OUTPUT_DIR}/$1"
    # Build the book first, because mdbook will create any empty sections
    # The PATH override lets it find our local copy of mdbook-mermaid
    PATH=$PATH:. ./mdbook build -d "${OUTPUT_DIR}/$1/book" ./presentations
    # Then build the slides
    RUST_LOG=info ./mdslides --template ./presentations/template.html \
        --output-dir "${OUTPUT_DIR}/$1/slides" \
        --mdbook-path ./presentations \
        --index-template ./presentations/index-template.html
    cp -r "${OUTPUT_DIR}/$1/book/images" "${OUTPUT_DIR}/$1/slides"
}

# Build what we currently have checked out
build_and_store latest

# Fetch all the git tags (in case this is some kind of shallow clone)
git fetch --tags

for tag in $(git tag); do
    echo "Building ${tag}..."
    # Make a place to put the rendered output
    mkdir -p "${OUTPUT_DIR}/${tag}"
    # Fetch a clean copy of the source material for this tag
    rm -rf ./presentations
    git checkout -f "${tag}"
    build_and_store "${tag}"
done
