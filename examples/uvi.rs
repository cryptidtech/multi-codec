// SPDX-License-Identifier: MIT OR Apache-2.0
#![allow(
    clippy::unreadable_literal,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::should_panic_without_expect,
    clippy::items_after_statements,
    clippy::match_same_arms
)]

//! Varuint encode/decode example using unsigned-varint crate.
//!
//! This example demonstrates encoding and decoding unsigned variable-length integers
//! according to the varuint specification (LEB128 encoding).
//!
//! Usage:
//! ```bash
//! # Encode a hex number to varuint (hex output)
//! cargo run --example uvi -- -e 012c
//!
//! # Decode a hex-encoded varuint
//! cargo run --example uvi -- -d ac02
//! ```

use std::env;
use std::process;

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  uvi -e <hex>        Encode a hex number as varuint (hex output)");
    eprintln!("  uvi -d <hex>        Decode a hex-encoded varuint");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  uvi -e 012c         # Encodes hex 0x012c (300) as varuint");
    eprintln!("  uvi -d ac02         # Decodes hex 'ac02' as varuint");
}

fn encode_hex(hex_str: &str) -> Result<(), String> {
    // Remove any whitespace or 0x prefix
    let cleaned = hex_str.trim().trim_start_matches("0x");

    let num = u64::from_str_radix(cleaned, 16)
        .map_err(|e| format!("Invalid hex number '{hex_str}': {e}"))?;

    let mut buf = unsigned_varint::encode::u64_buffer();
    let encoded = unsigned_varint::encode::u64(num, &mut buf);

    let hex_output = hex::encode(encoded);
    println!("{hex_output}");

    Ok(())
}

fn decode_hex(hex_str: &str) -> Result<(), String> {
    // Remove any whitespace or 0x prefix
    let cleaned = hex_str.trim().trim_start_matches("0x");

    let bytes = hex::decode(cleaned).map_err(|e| format!("Invalid hex string '{hex_str}': {e}"))?;

    let (num, remaining) = unsigned_varint::decode::u64(&bytes)
        .map_err(|e| format!("Failed to decode varuint: {e:?}"))?;

    if !remaining.is_empty() {
        eprintln!("Warning: {} extra bytes after varuint", remaining.len());
    }

    // Output as hex with decimal in parentheses
    println!("{num:x} ({num})");

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        print_usage();
        process::exit(1);
    }

    let result = match args[1].as_str() {
        "-e" | "--encode" => encode_hex(&args[2]),
        "-d" | "--decode" => decode_hex(&args[2]),
        _ => {
            print_usage();
            process::exit(1);
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
