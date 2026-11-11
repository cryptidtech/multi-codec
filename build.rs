// SPDX-License-Identifier: MIT or Apache-2.0
//! Build script for multi-codec
//!
//! Generates the Codec enum from the multicodec table CSV file.

use convert_case::{Case, Converter};
use serde_derive::Deserialize;
use std::{collections::HashSet, fs::File, io::Write, path::PathBuf};

#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    #[serde(rename = "tag")]
    _tag: String,
    code: String,
    #[serde(rename = "status")]
    _status: String,
    #[serde(rename = "description")]
    _description: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=table.csv");

    // Input path (table.csv lives in the crate root).
    let input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("table.csv");

    // The generated file must go into Cargo's OUT_DIR rather than
    // the source tree since docs.rs mounts the src directory as read-only
    let output_path = PathBuf::from(std::env::var("OUT_DIR")?).join("table_gen.rs");

    // Open input file
    let inf = File::open(&input_path).map_err(|e| {
        format!("Failed to open table.csv: {e}. Make sure table.csv exists in the crate root.")
    })?;

    // Open output file
    let mut f =
        File::create(&output_path).map_err(|e| format!("Failed to create table_gen.rs: {e}"))?;

    let mut rdr = csv::Reader::from_reader(inf);
    let conv = Converter::new().to_case(Case::Pascal);

    // Track seen codes and names to detect duplicates
    let mut seen_codes = HashSet::new();
    let mut seen_names = HashSet::new();
    let mut record_count = 0;

    writeln!(f, "build_codec_enum! {{")?;

    for (row_num, row) in rdr.deserialize().enumerate() {
        let rec: Record =
            row.map_err(|e| format!("Failed to parse CSV row {}: {}", row_num + 2, e))?;

        // Validate record
        let code_str = rec.code.trim();
        if code_str.is_empty() {
            return Err(format!("Empty code at row {}", row_num + 2).into());
        }

        if rec.name.is_empty() {
            return Err(format!("Empty name at row {}", row_num + 2).into());
        }

        // Check for duplicates — duplicate codes/names produce ambiguous enum
        // variants and must fail the build rather than silently emitting a
        // warning. The Rust compiler would eventually reject duplicate match
        // arms, but failing early in the build script gives a clearer error
        // pointing at the offending CSV row.
        if !seen_codes.insert(code_str.to_string()) {
            return Err(format!(
                "Duplicate code {} at row {} in table.csv — each multicodec code must be unique",
                code_str,
                row_num + 2
            )
            .into());
        }

        if !seen_names.insert(rec.name.clone()) {
            return Err(format!(
                "Duplicate name '{}' at row {} in table.csv — each multicodec name must be unique",
                rec.name,
                row_num + 2
            )
            .into());
        }

        // Generate enum variant
        let variant_name = conv.convert(&rec.name);
        // Classic McEliece key material is deprecated. Key-recovery attacks now
        // solve the TII McEliece challenges, so every McEliece variant carries a
        // deprecation attribute; uses emit a compiler warning. The variants stay
        // compiled and decodable so stored multicodec-tagged data keeps working.
        // See https://github.com/mjosaarinen/tii-solved for the recovered keys.
        // The attribute precedes the variant name inside the tuple so the
        // macro's `expr` fragment stays unambiguous.
        if rec.name.contains("mceliece") {
            writeln!(
                f,
                "\t{} => (#[deprecated(since = \"1.5.0\", note = \"Classic McEliece key recovery attacks: see https://github.com/mjosaarinen/tii-solved\")] {}, \"{}\"),",
                code_str, variant_name, rec.name
            )?;
        } else {
            writeln!(f, "\t{} => ({}, \"{}\"),", code_str, variant_name, rec.name)?;
        }

        record_count += 1;
    }

    writeln!(f, "}}")?;

    // Flush to ensure all data is written
    f.flush()?;

    // Informational: Generated codec variants (removed warning output for cleaner builds)
    // Use RUST_LOG=debug or --verbose to see build script output if needed
    // eprintln!("Generated {} codec variants from table.csv", record_count);

    if record_count == 0 {
        return Err("No records found in table.csv!".into());
    }

    Ok(())
}
