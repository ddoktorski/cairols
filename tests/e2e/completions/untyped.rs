use lsp_types::request::Completion;

use crate::{completions::completion_fixture, support::insta::test_transform_plain};

#[test]
fn no_text_in_function_context() {
    test_transform_plain!(Completion, completion_fixture(), "
    struct MyStruct {}

    fn a() {
        <caret>
    }
    ",@r#"
    caret = """
        <caret>
    """
    completions = []
    "#);
}

#[test]
fn no_text_after_semicolon() {
    test_transform_plain!(Completion, completion_fixture(), "
    struct MyStruct {}

    fn a() {
        let _x = 1;<caret>
    }
    ",@r#"
    caret = """
        let _x = 1;<caret>
    """
    completions = []
    "#);
}

#[test]
fn no_text_before_statement() {
    test_transform_plain!(Completion, completion_fixture(), "
    struct MyStruct {}

    fn a() {
        <caret>let _x = 1;
    }
    ",@r#"
    caret = """
        <caret>let _x = 1;
    """
    completions = []
    "#);
}

#[test]
fn no_text_after_statement() {
    test_transform_plain!(Completion, completion_fixture(), "
    struct MyStruct {}

    fn a() {
        let _x = 1;
        <caret>
        let _y = 2;
    }
    ",@r#"
    caret = """
        <caret>
    """

    [[completions]]
    completion_label = "_x"
    completion_label_type_info = "felt252"
    "#);
}
