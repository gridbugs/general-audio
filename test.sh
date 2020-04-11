#!/bin/bash

set -euxo pipefail

export RUSTFLAGS="--deny warnings"

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

cargo clean

cargo test --workspace
cargo test --manifest-path audio/Cargo.toml --all-features
cargo test --manifest-path native/Cargo.toml --all-features
cargo test --manifest-path web/Cargo.toml --all-features

cargo clean
