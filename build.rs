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

    let mut pb = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut tpb = pb.clone();

    // Input path
    tpb.push("table.csv");

    // Output path
    pb.push("src");
    pb.push("table_gen.rs");

    // Open input file
    let inf = File::open(&tpb).map_err(|e| {
        format!(
            "Failed to open table.csv: {}. Make sure table.csv exists in the crate root.",
            e
        )
    })?;

    // Open output file
    let mut f = File::create(&pb).map_err(|e| format!("Failed to create table_gen.rs: {}", e))?;

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

        // Check for duplicates
        if !seen_codes.insert(code_str.to_string()) {
            eprintln!(
                "Warning: Duplicate code {} at row {}",
                code_str,
                row_num + 2
            );
        }

        if !seen_names.insert(rec.name.clone()) {
            eprintln!(
                "Warning: Duplicate name '{}' at row {}",
                rec.name,
                row_num + 2
            );
        }

        // Generate enum variant
        let variant_name = conv.convert(&rec.name);
        writeln!(f, "\t{} => ({}, \"{}\"),", code_str, variant_name, rec.name)?;

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
