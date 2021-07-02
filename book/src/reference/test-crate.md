# The test Crate

The following example (from the smithy crate) shows how the common test crate is used to read a `.smithy` file, then 
serialize in the line-oriented form and compare to a pre-stored expected result.

```rust
use atelier_smithy::SmithyReader;
use atelier_test::parse_and_compare_to_files;
use std::path::PathBuf;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn test_file_parses(file_name: &str) {
    let source_file = PathBuf::from(format!("{}/tests/good/{}.smithy", MANIFEST_DIR, file_name));
    let expected_file = PathBuf::from(format!("{}/tests/good/{}.lines", MANIFEST_DIR, file_name));
    let mut reader = SmithyReader::default();
    parse_and_compare_to_files(&mut reader, &source_file, &expected_file);
}

#[test]
fn test_weather_example() {
    test_file_parses("weather");
}
```

For more information, see the [crate documentation](https://docs.rs/atelier_test/).