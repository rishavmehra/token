//! Another ERC20-like Token program for the Solana blockchain.

#![no_std]

// upstream
#[cfg(target_arch = "bpf")]
extern crate solana_compiler_builtins as _;

mod entrypoint;
mod processor;
