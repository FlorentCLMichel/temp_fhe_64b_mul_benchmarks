#!/bin/sh

# Install the Rust toolchain if needed
if [ "$(command -v rustc)" ]; then
  echo "Rust toolchain already installed.";
else
  echo "Installing the Rust toolchain...";
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh;
fi

# Build he submission
cd implementation_0_tfhe_rs
cargo build --release
