use atelier_core::io::write_model_to_string;
use atelier_describe::document::writer::DocumentationWriter;
use atelier_test::examples::motd::make_message_of_the_day_model;

const EXPECTED: &[&str] = &[
    "[_metadata_:title]:- \"Smithy Model\"",
    "Smithy Version: 1.0",
    "# Namespace example.motd",
    "## BadDateValue (structure)",
    "<table>",
    "<tr><th>Trait ID</th><th>Path</th><th>Value</th></tr>",
    "<tr><td>error</td><td></td><td>`client`</td></tr>",
    "</table>",
    "### Members",
    "<table>",
    "<tr><th>Trait ID</th><th>Path</th><th>Value</th></tr>",
    "<tr><td>required</td><td></td><td></td></tr>",
    "</table>",
    "> `errorMessage`: `smithy.api#String`",
    "> ",
    "## Date (string)",
    "<table>",
    "<tr><th>Trait ID</th><th>Path</th><th>Value</th></tr>",
    "<tr><td>pattern</td><td></td><td>`^\\d\\d\\d\\d\\-\\d\\d-\\d\\d$`</td></tr>",
    "</table>",
    "## GetMessage (operation)",
    "<table>",
    "<tr><th>Trait ID</th><th>Path</th><th>Value</th></tr>",
    "<tr><td>readonly</td><td></td><td></td></tr>",
    "</table>",
    "> **Input type**: [GetMessageInput](#shape:GetMessageInput)",
    "> ",
    "> **Output type**: [GetMessageInput](#shape:GetMessageInput)",
    "> ",
    "> **Errors**:",
    "> ",
    "> * [BadDateValue](#shape:BadDateValue)",
    "> ",
    "## GetMessageInput (structure)",
    "### Members",
    "> `date`: [Date](#shape:Date)",
    "> ",
    "## GetMessageOutput (structure)",
    "### Members",
    "<table>",
    "<tr><th>Trait ID</th><th>Path</th><th>Value</th></tr>",
    "<tr><td>required</td><td></td><td></td></tr>",
    "</table>",
    "> `message`: `smithy.api#String`",
    "> ",
    "## Message (resource)",
    "### Resource Operations",
    "> * `read`: [GetMessage](#shape:GetMessage)",
    "> ",
    "## MessageOfTheDay (service)",
    "Provides a Message of the day.",
    "**Service version**: 2020-06-21",
    "### Resources",
    "> * [Message](#shape:Message)",
    "> ",
];
#[test]
pub fn test_documentation_writer() {
    let model = make_message_of_the_day_model().model;
    let mut writer = DocumentationWriter::default();
    let out_str: Vec<String> = write_model_to_string(&mut writer, &model)
        .unwrap()
        .split('\n')
        .map(str::to_string)
        .filter(|s| !s.trim().is_empty())
        .collect();
    println!("{:#?}", out_str);
    assert_eq!(out_str, EXPECTED)
}
