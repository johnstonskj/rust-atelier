use atelier_core::io::lines::make_line_oriented_form;
use atelier_test::examples::{motd::make_message_of_the_day_model, weather::make_weather_model};
use atelier_test::parts;
use pretty_assertions::assert_eq;

#[test]
fn test_motd_to_lines() {
    let test_case = make_message_of_the_day_model();
    let lines = make_line_oriented_form(&test_case.model);
    println!("{:#?}", lines);
    assert_eq!(lines, test_case.expected_lines);
}

#[test]
fn test_weather_to_lines() {
    let test_case = make_weather_model();
    let lines = make_line_oriented_form(&test_case.model);
    println!("{:#?}", lines);
    assert_eq!(lines, test_case.expected_lines);
}

#[test]
fn namespace_only() {
    let test_case = parts::namespace_only();
    let lines = make_line_oriented_form(&test_case.model);
    println!("{:#?}", lines);
    assert_eq!(lines, test_case.expected_lines);
}

#[test]
fn simple_shape_only() {
    let test_case = parts::simple_shape_only();
    let lines = make_line_oriented_form(&test_case.model);
    println!("{:#?}", lines);
    assert_eq!(lines, test_case.expected_lines);
}

#[test]
fn simple_shape_with_block_text() {
    let test_case = parts::simple_shape_with_block_text();
    let lines = make_line_oriented_form(&test_case.model);
    println!("{:#?}", lines);
    assert_eq!(lines, test_case.expected_lines);
}

#[test]
fn simple_shape_with_block_text_2() {
    let test_case = parts::simple_shape_with_block_text_2();
    let lines = make_line_oriented_form(&test_case.model);
    println!("{:#?}", lines);
    assert_eq!(lines, test_case.expected_lines);
}

#[test]
fn simple_shape_with_traits() {
    let test_case = parts::simple_shape_with_traits();
    let lines = make_line_oriented_form(&test_case.model);
    println!("{:#?}", lines);
    assert_eq!(lines, test_case.expected_lines);
}

#[test]
fn simple_shape_with_traits_and_comments() {
    let test_case = parts::simple_shape_with_traits_and_comments();
    let lines = make_line_oriented_form(&test_case.model);
    println!("{:#?}", lines);
    assert_eq!(lines, test_case.expected_lines);
}

#[test]
fn simple_shape_with_traits_and_documentation() {
    let test_case = parts::simple_shape_with_traits_and_documentation();
    let lines = make_line_oriented_form(&test_case.model);
    println!("{:#?}", lines);
    assert_eq!(lines, test_case.expected_lines);
}

#[test]
fn service_with_renames() {
    let test_case = parts::service_with_renames();
    let lines = make_line_oriented_form(&test_case.model);
    println!("{:#?}", lines);
    assert_eq!(lines, test_case.expected_lines);
}
