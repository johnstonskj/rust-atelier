use atelier_core::io::lines::make_line_oriented_form;
use atelier_core::io::{read_model_from_string, ModelReader};
use atelier_core::model::Model;
use pretty_assertions::assert_eq;
use std::fs;
use std::path::Path;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// A Type that represents the result of the `LineOrientedWriter` output.
pub type ExpectedLines = Vec<&'static str>;

pub struct TestCaseModel {
    pub model: Model,
    pub expected_lines: ExpectedLines,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn parse_and_compare_model(input_str: &str, reader: &mut impl ModelReader, expected: Model) {
    println!("input to parse:\n{}", input_str);
    horizontal_line();
    match read_model_from_string(reader, input_str) {
        Ok(actual) => {
            println!("actual:\n{:#?}", actual);
            horizontal_line();
            assert_eq!(actual, expected);
        }
        Err(err) => panic!("error: {:#?}", err),
    }
}

pub fn parse_and_compare_to_file(input_str: &str, reader: &mut impl ModelReader, file_path: &Path) {
    println!("input to parse:\n{}", input_str);
    horizontal_line();
    match read_model_from_string(reader, input_str) {
        Ok(actual) => {
            println!("actual:\n{:#?}", actual);
            horizontal_line();
            compare_model_to_file(actual, file_path);
        }
        Err(err) => panic!("error: {:#?}", err),
    }
}

#[cfg(windows)]
const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &str = "\n";

pub fn compare_model_to_file(model: Model, file_path: &Path) {
    let expected_lines = fs::read_to_string(file_path).unwrap();

    let actual_lines = make_line_oriented_form(&model).join(LINE_ENDING);

    assert_eq!(actual_lines, expected_lines);
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline]
fn horizontal_line() {
    println!("------------------------------------------------------------------------------------------------");
}
// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod parts;

pub mod examples;
