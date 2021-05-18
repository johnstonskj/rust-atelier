use atelier_core::io::write_model_to_string;
use atelier_core::model::Model;
use atelier_describe::document::writer::DocumentationWriter;
use atelier_test::parts;

pub fn write_model(model: Model, expected: &[&str]) {
    let mut writer = DocumentationWriter::default();
    let out_str: Vec<String> = write_model_to_string(&mut writer, &model)
        .unwrap()
        .split('\n')
        .map(str::to_string)
        .filter(|s| !s.trim().is_empty())
        .collect();
    println!("{:#?}", out_str);
    assert_eq!(out_str, expected)
}

#[test]
fn namespace_only() {
    let model = parts::namespace_only();
    write_model(
        model.model,
        &[
            "[_metadata_:title]:- \"Smithy Model\"",
            "Smithy Version: 1.0",
        ],
    );
}

#[test]
fn simple_shape_only() {
    let model = parts::simple_shape_only();
    write_model(
        model.model,
        &[
            "[_metadata_:title]:- \"Smithy Model\"",
            "Smithy Version: 1.0",
            "# Namespace smithy.waiters",
            "## WaiterDelay (integer)",
        ],
    );
}

#[test]
fn simple_shape_with_block_text() {
    let model = parts::namespace_only();
    write_model(
        model.model,
        &[
            "[_metadata_:title]:- \"Smithy Model\"",
            "Smithy Version: 1.0",
        ],
    );
}

#[test]
fn simple_shape_with_block_text_2() {
    let model = parts::simple_shape_with_block_text_2();
    write_model(
        model.model,
        &[
            "[_metadata_:title]:- \"Smithy Model\"",
            "Smithy Version: 1.0",
            "# Namespace example.foo",
            "## FooDelay (integer)",
            "Do empty \"\" quotes work too?",
        ],
    );
}

#[test]
fn simple_shape_with_traits() {
    let model = parts::simple_shape_with_traits();
    write_model(
        model.model,
        &[
            "[_metadata_:title]:- \"Smithy Model\"",
            "Smithy Version: 1.0",
            "# Namespace smithy.waiters",
            "## WaiterDelay (integer)",
            "<table>",
            "<tr><th>Trait ID</th><th>Path</th><th>Value</th></tr>",
            "<tr><td>box</td><td></td><td></td></tr>",
            "<tr><td>range</td><td>`.min`</td><td>`1`</td></tr>",
            "</table>",
        ],
    );
}

#[test]
fn simple_shape_with_traits_and_comments() {
    let model = parts::simple_shape_with_traits_and_comments();
    write_model(
        model.model,
        &[
            "[_metadata_:title]:- \"Smithy Model\"",
            "Smithy Version: 1.0",
            "# Namespace smithy.waiters",
            "## WaiterDelay (integer)",
            "<table>",
            "<tr><th>Trait ID</th><th>Path</th><th>Value</th></tr>",
            "<tr><td>box</td><td></td><td></td></tr>",
            "<tr><td>range</td><td>`.min`</td><td>`1`</td></tr>",
            "</table>",
        ],
    );
}

#[test]
fn simple_shape_with_traits_and_documentation() {
    let model = parts::simple_shape_with_traits_and_documentation();
    write_model(
        model.model,
        &[
            "[_metadata_:title]:- \"Smithy Model\"",
            "Smithy Version: 1.0",
            "# Namespace smithy.waiters",
            "## WaiterName (string)",
            "The name, or identifier, of a waiter.",
            "<table>",
            "<tr><th>Trait ID</th><th>Path</th><th>Value</th></tr>",
            "<tr><td>pattern</td><td></td><td>`^[A-Z]+[A-Za-z0-9]*$`</td></tr>",
            "</table>",
        ],
    );
}

#[test]
fn service_with_renames() {
    let model = parts::service_with_renames();
    write_model(
        model.model,
        &[
            "[_metadata_:title]:- \"Smithy Model\"",
            "Smithy Version: 1.0",
            "# Namespace smithy.example",
            "## GetSomething (operation)",
            "## MyService (service)",
            "**Service version**: 2017-02-11",
            "### Operations",
            "> * [GetSomething](#shape:GetSomething)",
            "> ",
            "### Renames",
            "> * `foo.example#Widget` renamed to `FooWidget`",
            "> ",
        ],
    );
}
