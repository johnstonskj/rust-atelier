/*!
This crate is contains common test cases for Atelier readers and writers.
*/

#![warn(
    // ---------- Stylistic
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    // ---------- Public
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    // ---------- Unsafe
    unsafe_code,
    // ---------- Unused
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
)]

use atelier_core::io::lines::make_line_oriented_form;
use atelier_core::io::{read_model_from_file, read_model_from_string, ModelReader};
use atelier_core::model::Model;
use pretty_assertions::assert_eq;
use std::fs;
use std::path::{Path, PathBuf};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A Type that represents the result of the `LineOrientedWriter` output.
///
pub type ExpectedLines = Vec<&'static str>;

///
/// A model that should match the expected lines in `LineOrientedWriter` format.
///
#[derive(Clone, Debug)]
pub struct TestCaseModel {
    /// The model to write
    pub model: Model,
    /// The expected result.
    pub expected_lines: ExpectedLines,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[cfg(windows)]
const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &str = "\n";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Parse a model from the string `input_str`, using the `reader` representation implementation, and then
/// compare to the `expected` model value.
///
pub fn parse_and_compare_model(input_str: &str, reader: &mut impl ModelReader, expected: Model) {
    println!("input to parse:\n{}", input_str);
    horizontal_line();
    match read_model_from_string(reader, input_str) {
        Ok(actual) => {
            horizontal_line();
            assert_eq!(actual, expected);
        }
        Err(err) => panic!("error: {:#?}", err),
    }
}

///
/// Parse the file in `actual_path`, using the `reader` representation implementation, and then
/// compare to the line-oriented representation in `expected_path`.
///
pub fn parse_and_compare_to_files(
    reader: &mut impl ModelReader,
    actual_path: &Path,
    expected_path: &Path,
) {
    let result = read_model_from_file(reader, PathBuf::from(actual_path));
    match result {
        Ok(model) => {
            compare_model_to_file(model, expected_path);
        }
        Err(err) => panic!("{}", err),
    }
}

///
/// Parse a model from the string `input_str`, using the `reader` representation implementation, and then
/// compare to the line-oriented representation in `expected_path`.
///
pub fn parse_and_compare_to_file(
    input_str: &str,
    reader: &mut impl ModelReader,
    expected_path: &Path,
) {
    horizontal_line();
    match read_model_from_string(reader, input_str) {
        Ok(actual) => {
            horizontal_line();
            compare_model_to_file(actual, expected_path);
        }
        Err(err) => panic!("error: {:#?}", err),
    }
}

///
/// Serialize `model` in the line-oriented representation and compare to the expected value in
/// `expected_path`.
///
pub fn compare_model_to_file(model: Model, expected_path: &Path) {
    let actual_lines: Vec<String> = make_line_oriented_form(&model)
        .iter()
        .map(|s| {
            format!(
                "{:?}",
                if s.contains("\r\n") {
                    s.replace("\r\n", "\n")
                } else {
                    s.to_string()
                }
            )
        })
        .collect();

    let expected_lines: Vec<String> = fs::read_to_string(expected_path)
        .unwrap()
        .split(LINE_ENDING)
        .map(|s| {
            format!(
                "{:?}",
                s.replace("\\n", "\n")
                    .replace("\\t", "\t")
                    .replace("\\\"", "\"")
            )
        })
        .collect();

    assert_eq!(actual_lines, expected_lines);
}

#[inline]
fn horizontal_line() {
    println!("------------------------------------------------------------------------------------------------");
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod parts;

pub mod examples;
