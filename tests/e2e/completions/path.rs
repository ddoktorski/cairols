use lsp_types::request::Completion;

use crate::{
    completions::completion_fixture,
    support::insta::{test_transform_plain, test_transform_with_macros},
};

#[test]
fn single_element_path() {
    test_transform_plain!(Completion, completion_fixture(), "
    struct ByteA_ActuallyNotByteArray {}

    fn a() {
        ByteA<caret>
    }
    ",@r#"
    caret = """
        ByteA<caret>
    """

    [[completions]]
    completion_label = "ByteA_ActuallyNotByteArray {...}"
    completion_label_path = "(use ByteA_ActuallyNotByteArray)"
    completion_label_type_info = "ByteA_ActuallyNotByteArray {}"
    insert_text = "ByteA_ActuallyNotByteArray {}"

    [[completions]]
    completion_label = "ByteArray"

    [[completions]]
    completion_label = "ByteArrayTrait"

    [[completions]]
    completion_label = "ByteArrayTrait::append(...)"
    completion_label_type_info = "fn(ref self: ByteArray, other: @ByteArray) -> ()"
    insert_text = "ByteArrayTrait::append(${1:other})"

    [[completions]]
    completion_label = "ByteArrayTrait::append_byte(...)"
    completion_label_type_info = "fn(ref self: ByteArray, byte: u8) -> ()"
    insert_text = "ByteArrayTrait::append_byte(${1:byte})"

    [[completions]]
    completion_label = "ByteArrayTrait::append_word(...)"
    completion_label_type_info = "fn(ref self: ByteArray, word: felt252, len: u32) -> ()"
    insert_text = "ByteArrayTrait::append_word(${1:word}, ${2:len})"

    [[completions]]
    completion_label = "ByteArrayTrait::append_word_rev(...)"
    completion_label_type_info = "fn(ref self: ByteArray, word: felt252, len: u32) -> ()"
    insert_text = "ByteArrayTrait::append_word_rev(${1:word}, ${2:len})"

    [[completions]]
    completion_label = "ByteArrayTrait::at(...)"
    completion_label_type_info = "fn(self: @ByteArray, index: u32) -> Option<u8>"
    insert_text = "ByteArrayTrait::at(${1:index})"

    [[completions]]
    completion_label = "ByteArrayTrait::concat(...)"
    completion_label_type_info = "fn(left: @ByteArray, right: @ByteArray) -> ByteArray"
    insert_text = "ByteArrayTrait::concat(${1:left}, ${2:right})"

    [[completions]]
    completion_label = "ByteArrayTrait::len(...)"
    completion_label_type_info = "fn(self: @ByteArray) -> u32"
    insert_text = "ByteArrayTrait::len()"

    [[completions]]
    completion_label = "ByteArrayTrait::rev(...)"
    completion_label_type_info = "fn(self: @ByteArray) -> ByteArray"
    insert_text = "ByteArrayTrait::rev()"

    [[completions]]
    completion_label = "Bytes31Trait"

    [[completions]]
    completion_label = "Bytes31Trait::at(...)"
    completion_label_type_info = "fn(self: @bytes31, index: u32) -> u8"
    insert_text = "Bytes31Trait::at(${1:index})"

    [[completions]]
    completion_label = "System"

    [[completions]]
    completion_label = "BitAnd"
    completion_label_path = "(use core::traits::BitAnd)"
    text_edits = ["""
    use core::traits::BitAnd;

    """]

    [[completions]]
    completion_label = "BitAnd::bitand(...)"
    completion_label_type_info = "fn(lhs: T, rhs: T) -> T"
    insert_text = "BitAnd::bitand(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::traits::BitAnd;

    """]

    [[completions]]
    completion_label = "ByteArrayImpl"
    completion_label_path = "(use core::byte_array::ByteArrayImpl)"
    text_edits = ["""
    use core::byte_array::ByteArrayImpl;

    """]

    [[completions]]
    completion_label = "ByteArrayImpl::append(...)"
    completion_label_type_info = "fn(ref self: ByteArray, other: @ByteArray) -> ()"
    insert_text = "ByteArrayImpl::append(${1:other})"
    text_edits = ["""
    use core::byte_array::ByteArrayImpl;

    """]

    [[completions]]
    completion_label = "ByteArrayImpl::append_byte(...)"
    completion_label_type_info = "fn(ref self: ByteArray, byte: u8) -> ()"
    insert_text = "ByteArrayImpl::append_byte(${1:byte})"
    text_edits = ["""
    use core::byte_array::ByteArrayImpl;

    """]

    [[completions]]
    completion_label = "ByteArrayImpl::append_word(...)"
    completion_label_type_info = "fn(ref self: ByteArray, word: felt252, len: u32) -> ()"
    insert_text = "ByteArrayImpl::append_word(${1:word}, ${2:len})"
    text_edits = ["""
    use core::byte_array::ByteArrayImpl;

    """]

    [[completions]]
    completion_label = "ByteArrayImpl::append_word_rev(...)"
    completion_label_type_info = "fn(ref self: ByteArray, word: felt252, len: u32) -> ()"
    insert_text = "ByteArrayImpl::append_word_rev(${1:word}, ${2:len})"
    text_edits = ["""
    use core::byte_array::ByteArrayImpl;

    """]

    [[completions]]
    completion_label = "ByteArrayImpl::at(...)"
    completion_label_type_info = "fn(self: @ByteArray, index: u32) -> Option<u8>"
    insert_text = "ByteArrayImpl::at(${1:index})"
    text_edits = ["""
    use core::byte_array::ByteArrayImpl;

    """]

    [[completions]]
    completion_label = "ByteArrayImpl::concat(...)"
    completion_label_type_info = "fn(left: @ByteArray, right: @ByteArray) -> ByteArray"
    insert_text = "ByteArrayImpl::concat(${1:left}, ${2:right})"
    text_edits = ["""
    use core::byte_array::ByteArrayImpl;

    """]

    [[completions]]
    completion_label = "ByteArrayImpl::len(...)"
    completion_label_type_info = "fn(self: @ByteArray) -> u32"
    insert_text = "ByteArrayImpl::len()"
    text_edits = ["""
    use core::byte_array::ByteArrayImpl;

    """]

    [[completions]]
    completion_label = "ByteArrayImpl::rev(...)"
    completion_label_type_info = "fn(self: @ByteArray) -> ByteArray"
    insert_text = "ByteArrayImpl::rev()"
    text_edits = ["""
    use core::byte_array::ByteArrayImpl;

    """]

    [[completions]]
    completion_label = "ByteArrayIter"
    completion_label_path = "(use core::byte_array::ByteArrayIter)"
    text_edits = ["""
    use core::byte_array::ByteArrayIter;

    """]

    [[completions]]
    completion_label = "ByteSpan"
    completion_label_path = "(use core::byte_array::ByteSpan)"
    text_edits = ["""
    use core::byte_array::ByteSpan;

    """]

    [[completions]]
    completion_label = "ByteSpanImpl"
    completion_label_path = "(use core::byte_array::ByteSpanImpl)"
    text_edits = ["""
    use core::byte_array::ByteSpanImpl;

    """]

    [[completions]]
    completion_label = "ByteSpanImpl::get(...)"
    completion_label_type_info = "fn(self: @ByteSpan, index: I) -> Option<TGet::Output>"
    insert_text = "ByteSpanImpl::get(${1:index})"
    text_edits = ["""
    use core::byte_array::ByteSpanImpl;

    """]

    [[completions]]
    completion_label = "ByteSpanImpl::is_empty(...)"
    completion_label_type_info = "fn(self: ByteSpan) -> bool"
    insert_text = "ByteSpanImpl::is_empty()"
    text_edits = ["""
    use core::byte_array::ByteSpanImpl;

    """]

    [[completions]]
    completion_label = "ByteSpanImpl::len(...)"
    completion_label_type_info = "fn(self: ByteSpan) -> u32"
    insert_text = "ByteSpanImpl::len()"
    text_edits = ["""
    use core::byte_array::ByteSpanImpl;

    """]

    [[completions]]
    completion_label = "ByteSpanImpl::to_byte_array(...)"
    completion_label_type_info = "fn(self: ByteSpan) -> ByteArray"
    insert_text = "ByteSpanImpl::to_byte_array()"
    text_edits = ["""
    use core::byte_array::ByteSpanImpl;

    """]

    [[completions]]
    completion_label = "ByteSpanIter"
    completion_label_path = "(use core::byte_array::ByteSpanIter)"
    text_edits = ["""
    use core::byte_array::ByteSpanIter;

    """]

    [[completions]]
    completion_label = "ByteSpanTrait"
    completion_label_path = "(use core::byte_array::ByteSpanTrait)"
    text_edits = ["""
    use core::byte_array::ByteSpanTrait;

    """]

    [[completions]]
    completion_label = "ByteSpanTrait::get(...)"
    completion_label_type_info = "fn(self: @ByteSpan, index: I) -> Option<TGet::Output>"
    insert_text = "ByteSpanTrait::get(${1:index})"
    text_edits = ["""
    use core::byte_array::ByteSpanTrait;

    """]

    [[completions]]
    completion_label = "ByteSpanTrait::is_empty(...)"
    completion_label_type_info = "fn(self: ByteSpan) -> bool"
    insert_text = "ByteSpanTrait::is_empty()"
    text_edits = ["""
    use core::byte_array::ByteSpanTrait;

    """]

    [[completions]]
    completion_label = "ByteSpanTrait::len(...)"
    completion_label_type_info = "fn(self: ByteSpan) -> u32"
    insert_text = "ByteSpanTrait::len()"
    text_edits = ["""
    use core::byte_array::ByteSpanTrait;

    """]

    [[completions]]
    completion_label = "ByteSpanTrait::to_byte_array(...)"
    completion_label_type_info = "fn(self: ByteSpan) -> ByteArray"
    insert_text = "ByteSpanTrait::to_byte_array()"
    text_edits = ["""
    use core::byte_array::ByteSpanTrait;

    """]

    [[completions]]
    completion_label = "Bytes31Impl"
    completion_label_path = "(use core::bytes_31::Bytes31Impl)"
    text_edits = ["""
    use core::bytes_31::Bytes31Impl;

    """]

    [[completions]]
    completion_label = "Bytes31Impl::at(...)"
    completion_label_type_info = "fn(self: @bytes31, index: u32) -> u8"
    insert_text = "Bytes31Impl::at(${1:index})"
    text_edits = ["""
    use core::bytes_31::Bytes31Impl;

    """]

    [[completions]]
    completion_label = "EcPointImpl::y(...)"
    completion_label_type_info = "fn(self: NonZero<EcPoint>) -> felt252"
    insert_text = "EcPointImpl::y()"
    text_edits = ["""
    use core::ec::EcPointImpl;

    """]

    [[completions]]
    completion_label = "EcPointTrait::y(...)"
    completion_label_type_info = "fn(self: NonZero<EcPoint>) -> felt252"
    insert_text = "EcPointTrait::y()"
    text_edits = ["""
    use core::ec::EcPointTrait;

    """]
    "#);
}

#[test]
fn multi_segment_path() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod foo {
        struct Bar {}
        pub struct Baz {}
    }

    fn a() {
        foo::B<caret>
    }
    ",@r#"
    caret = """
        foo::B<caret>
    """

    [[completions]]
    completion_label = "Baz {...}"
    completion_label_path = "(use foo::Baz)"
    completion_label_type_info = "Baz {}"
    insert_text = "Baz {}"
    "#);
}

#[test]
fn multi_segment_path_partial() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod foo {
        pub mod bar {
            pub struct Baz {}
        }
        pub struct Boo {}
    }

    fn a() {
        bar::B<caret>
    }
    ",@r#"
    caret = """
        bar::B<caret>
    """

    [[completions]]
    completion_label = "Baz {...}"
    completion_label_path = "(use foo::bar::Baz)"
    completion_label_type_info = "Baz {}"
    insert_text = "Baz {}"
    text_edits = ["""
    use foo::bar;

    """]
    "#);
}

#[test]
fn multi_segment_path_partial_macro() {
    test_transform_with_macros!(Completion, completion_fixture(), "
    mod foo {
        pub mod bar {
            pub struct Baz {}
        }
        pub struct Boo {}
    }

    #[complex_attribute_macro_v2]
    fn a() {
        bar::B<caret>
    }
    ",@r#"
    caret = """
        bar::B<caret>
    """

    [[completions]]
    completion_label = "Baz {...}"
    completion_label_path = "(use foo::bar::Baz)"
    completion_label_type_info = "Baz {}"
    insert_text = "Baz {}"
    text_edits = ["""
    use foo::bar;

    """]
    "#);
}

#[test]
fn enum_variant() {
    test_transform_plain!(Completion, completion_fixture(), "
    enum Enumik {
        A,
        B,
    }

    fn func() {
        let x = Enumik::<caret>
    }
    ",@r#"
    caret = """
        let x = Enumik::<caret>
    """

    [[completions]]
    completion_label = "A"
    completion_label_path = "(use Enumik::A)"

    [[completions]]
    completion_label = "B"
    completion_label_path = "(use Enumik::B)"
    "#);
}

#[test]
fn type_annotation() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod module {
        pub type felt = felt252;
        pub type int = i32;
        type priv_int = i32;
    }
    fn foo() {
        let x: module::<caret> = 0x0;
    }
    ",@r#"
    caret = """
        let x: module::<caret> = 0x0;
    """

    [[completions]]
    completion_label = "felt"
    completion_label_path = "(use module::felt)"

    [[completions]]
    completion_label = "int"
    completion_label_path = "(use module::int)"
    "#);
}

#[test]
fn type_annotation_with_dangling_path() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod module {
        pub type felt = felt252;
        pub type int = i32;
        type priv_int = i32;

        pub const CONST: u32 = 0;

        pub mod nested_module {
            pub type T = u32;
        }
    }
    fn foo() -> u32 {
        let x: module::<caret>
            nested_module::T = 0x0;
    }
    ",@r#"
    caret = """
        let x: module::<caret>
    """

    [[completions]]
    completion_label = "CONST"
    completion_label_path = "(use module::CONST)"

    [[completions]]
    completion_label = "felt"
    completion_label_path = "(use module::felt)"

    [[completions]]
    completion_label = "int"
    completion_label_path = "(use module::int)"

    [[completions]]
    completion_label = "nested_module"
    completion_label_path = "(use module::nested_module)"
    "#);
}

#[test]
fn type_annotation_with_trivia() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod module {
        pub type felt = felt252;
        pub type int = i32;
        type priv_int = i32;
    }
    fn foo() {
        let x: module::<caret> // comment
            = 0x0;
    }
    ",@r#"
    caret = """
        let x: module::<caret> // comment
    """

    [[completions]]
    completion_label = "felt"
    completion_label_path = "(use module::felt)"

    [[completions]]
    completion_label = "int"
    completion_label_path = "(use module::int)"
    "#);
}

#[test]
fn generic_parameter() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod module {
        pub type felt = felt252;
        pub type int = i32;
        type priv_int = i32;
    }
    fn foo() {
        let x = Into::<module::<caret>, u32>(0);
    }
    ",@r#"
    caret = """
        let x = Into::<module::<caret>, u32>(0);
    """

    [[completions]]
    completion_label = "felt"
    completion_label_path = "(use module::felt)"

    [[completions]]
    completion_label = "int"
    completion_label_path = "(use module::int)"
    "#);
}

#[test]
fn generic_parameter_with_trivia() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod module {
        pub type felt = felt252;
        pub type int = i32;
        type priv_int = i32;
    }
    fn foo() {
        let x = Into::<module::<caret>//comment
        , u32>(0);
    }
    ",@r#"
    caret = """
        let x = Into::<module::<caret>//comment
    """

    [[completions]]
    completion_label = "felt"
    completion_label_path = "(use module::felt)"

    [[completions]]
    completion_label = "int"
    completion_label_path = "(use module::int)"
    "#);
}

#[test]
fn function_implicit_parameter() {
    test_transform_plain!(Completion, completion_fixture(), "
    fn foo() implicits(core::Range<caret>) {}
    ",@r#"
    caret = """
    fn foo() implicits(core::Range<caret>) {}
    """

    [[completions]]
    completion_label = "RangeCheck"
    completion_label_path = "(use core::RangeCheck)"
    "#);
}

#[test]
fn simple_completion_without_explicit_path() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod a {
        pub fn xyz() {}
    }

    fn foo() {
        xy<caret>
    }
    ",@r#"
    caret = """
        xy<caret>
    """

    [[completions]]
    completion_label = "xyz(...)"
    completion_label_path = "(use a::xyz)"
    completion_label_type_info = "fn() -> ()"
    insert_text = "xyz()"
    text_edits = ["""
    use a::xyz;

    """]

    [[completions]]
    completion_label = "EcPointImpl::x(...)"
    completion_label_type_info = "fn(self: NonZero<EcPoint>) -> felt252"
    insert_text = "EcPointImpl::x()"
    text_edits = ["""
    use core::ec::EcPointImpl;

    """]

    [[completions]]
    completion_label = "EcPointTrait::x(...)"
    completion_label_type_info = "fn(self: NonZero<EcPoint>) -> felt252"
    insert_text = "EcPointTrait::x()"
    text_edits = ["""
    use core::ec::EcPointTrait;

    """]
    "#);
}

#[test]
fn duplicated_completion_without_explicit_path() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod a {
        pub fn xyz() {}
    }

    mod b {
        pub fn xyz() {}
    }

    fn foo() {
        xy<caret>
    }
    ",@r#"
    caret = """
        xy<caret>
    """

    [[completions]]
    completion_label = "xyz(...)"
    completion_label_path = "(use a::xyz)"
    completion_label_type_info = "fn() -> ()"
    insert_text = "xyz()"
    text_edits = ["""
    use a::xyz;

    """]

    [[completions]]
    completion_label = "xyz(...)"
    completion_label_path = "(use b::xyz)"
    completion_label_type_info = "fn() -> ()"
    insert_text = "xyz()"
    text_edits = ["""
    use b::xyz;

    """]

    [[completions]]
    completion_label = "EcPointImpl::x(...)"
    completion_label_type_info = "fn(self: NonZero<EcPoint>) -> felt252"
    insert_text = "EcPointImpl::x()"
    text_edits = ["""
    use core::ec::EcPointImpl;

    """]

    [[completions]]
    completion_label = "EcPointTrait::x(...)"
    completion_label_type_info = "fn(self: NonZero<EcPoint>) -> felt252"
    insert_text = "EcPointTrait::x()"
    text_edits = ["""
    use core::ec::EcPointTrait;

    """]
    "#);
}

#[test]
fn no_text_last_segment_in_function_context() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod my_mod {
       pub const MY_CONST: u8 = 5;
       pub fn my_func() {}
    }

    fn a() {
        my_mod::<caret>
    }
    ",@r#"
    caret = """
        my_mod::<caret>
    """

    [[completions]]
    completion_label = "MY_CONST"
    completion_label_path = "(use my_mod::MY_CONST)"

    [[completions]]
    completion_label = "my_func(...)"
    completion_label_path = "(use my_mod::my_func)"
    completion_label_type_info = "fn() -> ()"
    insert_text = "my_func()"
    "#);
}

#[test]
fn simple_declarative_macro_completion() {
    test_transform_plain!(Completion, completion_fixture(), "
    macro my_own_macro {
        ($x:ident) => {
            1
        };
    }

    fn foo() {
        let _a = my_own<caret>
    }
    ",@r#"
    caret = """
        let _a = my_own<caret>
    """

    [[completions]]
    completion_label = "my_own_macro!"
    completion_label_path = "(use my_own_macro)"
    insert_text = "my_own_macro!($1)"

    [[completions]]
    completion_label = "OptionTrait::map_or(...)"
    completion_label_type_info = "fn(self: Option<T>, default: U, f: F) -> U"
    insert_text = "OptionTrait::map_or(${1:default}, ${2:f})"

    [[completions]]
    completion_label = "ResultTrait::map_or(...)"
    completion_label_type_info = "fn(self: Result<T, E>, default: U, f: F) -> U"
    insert_text = "ResultTrait::map_or(${1:default}, ${2:f})"

    [[completions]]
    completion_label = "EcPointImpl::y(...)"
    completion_label_type_info = "fn(self: NonZero<EcPoint>) -> felt252"
    insert_text = "EcPointImpl::y()"
    text_edits = ["""
    use core::ec::EcPointImpl;

    """]

    [[completions]]
    completion_label = "EcPointTrait::y(...)"
    completion_label_type_info = "fn(self: NonZero<EcPoint>) -> felt252"
    insert_text = "EcPointTrait::y()"
    text_edits = ["""
    use core::ec::EcPointTrait;

    """]

    [[completions]]
    completion_label = "OptionTraitImpl::map_or(...)"
    completion_label_type_info = "fn(self: Option<T>, default: U, f: F) -> U"
    insert_text = "OptionTraitImpl::map_or(${1:default}, ${2:f})"
    text_edits = ["""
    use core::option::OptionTraitImpl;

    """]

    [[completions]]
    completion_label = "ResultTraitImpl::map_or(...)"
    completion_label_type_info = "fn(self: Result<T, E>, default: U, f: F) -> U"
    insert_text = "ResultTraitImpl::map_or(${1:default}, ${2:f})"
    text_edits = ["""
    use core::result::ResultTraitImpl;

    """]
    "#);
}

#[test]
fn declarative_macro_completion_without_explicit_path() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod my_mod {
        pub macro my_own_macro {
            ($x:ident) => {
                1
            };
        }
    }

    fn foo() {
        let _a = my_own<caret>
    }
    ",@r#"
    caret = """
        let _a = my_own<caret>
    """

    [[completions]]
    completion_label = "my_mod"

    [[completions]]
    completion_label = "OptionTrait::map_or(...)"
    completion_label_type_info = "fn(self: Option<T>, default: U, f: F) -> U"
    insert_text = "OptionTrait::map_or(${1:default}, ${2:f})"

    [[completions]]
    completion_label = "ResultTrait::map_or(...)"
    completion_label_type_info = "fn(self: Result<T, E>, default: U, f: F) -> U"
    insert_text = "ResultTrait::map_or(${1:default}, ${2:f})"

    [[completions]]
    completion_label = "my_own_macro!"
    completion_label_path = "(use my_mod::my_own_macro)"
    insert_text = "my_own_macro!($1)"
    text_edits = ["""
    use my_mod::my_own_macro;

    """]

    [[completions]]
    completion_label = "EcPointImpl::y(...)"
    completion_label_type_info = "fn(self: NonZero<EcPoint>) -> felt252"
    insert_text = "EcPointImpl::y()"
    text_edits = ["""
    use core::ec::EcPointImpl;

    """]

    [[completions]]
    completion_label = "EcPointTrait::y(...)"
    completion_label_type_info = "fn(self: NonZero<EcPoint>) -> felt252"
    insert_text = "EcPointTrait::y()"
    text_edits = ["""
    use core::ec::EcPointTrait;

    """]

    [[completions]]
    completion_label = "OptionTraitImpl::map_or(...)"
    completion_label_type_info = "fn(self: Option<T>, default: U, f: F) -> U"
    insert_text = "OptionTraitImpl::map_or(${1:default}, ${2:f})"
    text_edits = ["""
    use core::option::OptionTraitImpl;

    """]

    [[completions]]
    completion_label = "ResultTraitImpl::map_or(...)"
    completion_label_type_info = "fn(self: Result<T, E>, default: U, f: F) -> U"
    insert_text = "ResultTraitImpl::map_or(${1:default}, ${2:f})"
    text_edits = ["""
    use core::result::ResultTraitImpl;

    """]
    "#);
}

#[test]
fn trait_prefix_with_function() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait MyTrait {
        fn my_func() -> u32;
    }

    fn test() {
        MyTrait::<caret>
    }
    ",@r#"
    caret = """
        MyTrait::<caret>
    """

    [[completions]]
    completion_label = "my_func(...)"
    completion_label_type_info = "fn() -> u32"
    insert_text = "my_func()"
    "#);
}

#[test]
fn trait_prefix_with_type() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait MyTrait {
        type MyType;
    }

    fn test() {
        MyTrait::<caret>
    }
    ",@r#"
    caret = """
        MyTrait::<caret>
    """

    [[completions]]
    completion_label = "MyType"
    "#);
}

#[test]
fn trait_prefix_with_constant() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait MyTrait {
        const MY_CONST: u32;
    }

    fn test() {
        MyTrait::<caret>
    }
    ",@r#"
    caret = """
        MyTrait::<caret>
    """

    [[completions]]
    completion_label = "MY_CONST"
    completion_label_type_info = "u32"
    "#);
}

#[test]
fn trait_prefix_with_all_items() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait MyTrait {
        fn my_func() -> u32;
        type MyType;
        const MY_CONST: u32;
    }

    fn test() {
        MyTrait::<caret>
    }
    ",@r#"
    caret = """
        MyTrait::<caret>
    """

    [[completions]]
    completion_label = "MY_CONST"
    completion_label_type_info = "u32"

    [[completions]]
    completion_label = "MyType"

    [[completions]]
    completion_label = "my_func(...)"
    completion_label_type_info = "fn() -> u32"
    insert_text = "my_func()"
    "#);
}

#[test]
fn impl_prefix_with_all_items() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait MyTrait {
        fn my_func() -> u32;
        type MyType;
        const MY_CONST: u32;
    }

    impl MyImpl of MyTrait {
        fn my_func() -> u32 { 0 }
        type MyType = u32;
        const MY_CONST: u32 = 5;
    }

    fn test() {
        MyImpl::<caret>
    }
    ",@r#"
    caret = """
        MyImpl::<caret>
    """

    [[completions]]
    completion_label = "MY_CONST"
    completion_label_type_info = "u32"

    [[completions]]
    completion_label = "MyType"

    [[completions]]
    completion_label = "my_func(...)"
    completion_label_type_info = "fn() -> u32"
    insert_text = "my_func()"
    "#);
}

#[test]
fn impl_item_suffix_by_impl_name() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait MyTrait {
        fn my_func() -> u32;
        type MyType;
        const MY_CONST: u32;
    }

    impl MyImpl of MyTrait {
        fn my_func() -> u32 { 0 }
        type MyType = u32;
        const MY_CONST: u32 = 5;
    }

    fn test() {
        MyImpl<caret>
    }
    ",@r#"
    caret = """
        MyImpl<caret>
    """

    [[completions]]
    completion_label = "MyImpl"

    [[completions]]
    completion_label = "MyImpl::MY_CONST"
    completion_label_type_info = "u32"

    [[completions]]
    completion_label = "MyImpl::MyType"

    [[completions]]
    completion_label = "MyImpl::my_func(...)"
    completion_label_type_info = "fn() -> u32"
    insert_text = "MyImpl::my_func()"

    [[completions]]
    completion_label = "MyTrait::MyType"

    [[completions]]
    completion_label = "ArrayImpl"
    completion_label_path = "(use core::array::ArrayImpl)"
    text_edits = ["""
    use core::array::ArrayImpl;

    """]

    [[completions]]
    completion_label = "ArrayImpl::append(...)"
    completion_label_type_info = "fn(ref self: Array<T>, value: T) -> () nopanic"
    insert_text = "ArrayImpl::append(${1:value})"
    text_edits = ["""
    use core::array::ArrayImpl;

    """]

    [[completions]]
    completion_label = "ArrayImpl::append_span(...)"
    completion_label_type_info = "fn(ref self: Array<T>, span: Span<T>) -> ()"
    insert_text = "ArrayImpl::append_span(${1:span})"
    text_edits = ["""
    use core::array::ArrayImpl;

    """]

    [[completions]]
    completion_label = "ArrayImpl::at(...)"
    completion_label_type_info = "fn(self: @Array<T>, index: u32) -> @T"
    insert_text = "ArrayImpl::at(${1:index})"
    text_edits = ["""
    use core::array::ArrayImpl;

    """]

    [[completions]]
    completion_label = "ArrayImpl::get(...)"
    completion_label_type_info = "fn(self: @Array<T>, index: u32) -> Option<Box<@T>>"
    insert_text = "ArrayImpl::get(${1:index})"
    text_edits = ["""
    use core::array::ArrayImpl;

    """]

    [[completions]]
    completion_label = "ArrayImpl::is_empty(...)"
    completion_label_type_info = "fn(self: @Array<T>) -> bool"
    insert_text = "ArrayImpl::is_empty()"
    text_edits = ["""
    use core::array::ArrayImpl;

    """]

    [[completions]]
    completion_label = "ArrayImpl::len(...)"
    completion_label_type_info = "fn(self: @Array<T>) -> u32"
    insert_text = "ArrayImpl::len()"
    text_edits = ["""
    use core::array::ArrayImpl;

    """]

    [[completions]]
    completion_label = "ArrayImpl::new(...)"
    completion_label_type_info = "fn() -> Array<T> nopanic"
    insert_text = "ArrayImpl::new()"
    text_edits = ["""
    use core::array::ArrayImpl;

    """]

    [[completions]]
    completion_label = "ArrayImpl::pop_front(...)"
    completion_label_type_info = "fn(ref self: Array<T>) -> Option<T> nopanic"
    insert_text = "ArrayImpl::pop_front()"
    text_edits = ["""
    use core::array::ArrayImpl;

    """]

    [[completions]]
    completion_label = "ArrayImpl::pop_front_consume(...)"
    completion_label_type_info = "fn(self: Array<T>) -> Option<(Array<T>, T)> nopanic"
    insert_text = "ArrayImpl::pop_front_consume()"
    text_edits = ["""
    use core::array::ArrayImpl;

    """]

    [[completions]]
    completion_label = "ArrayImpl::span(...)"
    completion_label_type_info = "fn(snapshot: @Array<T>) -> Span<T>"
    insert_text = "ArrayImpl::span(${1:snapshot})"
    text_edits = ["""
    use core::array::ArrayImpl;

    """]

    [[completions]]
    completion_label = "Bounded::MIN"
    completion_label_type_info = "T"
    text_edits = ["""
    use core::num::traits::Bounded;

    """]

    [[completions]]
    completion_label = "BoxImpl"
    completion_label_path = "(use core::box::BoxImpl)"
    text_edits = ["""
    use core::box::BoxImpl;

    """]

    [[completions]]
    completion_label = "BoxImpl::as_snapshot(...)"
    completion_label_type_info = "fn(self: @Box<T>) -> Box<@T> nopanic"
    insert_text = "BoxImpl::as_snapshot()"
    text_edits = ["""
    use core::box::BoxImpl;

    """]

    [[completions]]
    completion_label = "BoxImpl::new(...)"
    completion_label_type_info = "fn(value: T) -> Box<T> nopanic"
    insert_text = "BoxImpl::new(${1:value})"
    text_edits = ["""
    use core::box::BoxImpl;

    """]

    [[completions]]
    completion_label = "BoxImpl::unbox(...)"
    completion_label_type_info = "fn(self: Box<T>) -> T nopanic"
    insert_text = "BoxImpl::unbox()"
    text_edits = ["""
    use core::box::BoxImpl;

    """]

    [[completions]]
    completion_label = "DebugImpl"
    completion_label_path = "(use core::fmt::into_felt252_based::DebugImpl)"
    text_edits = ["""
    use core::fmt::into_felt252_based::DebugImpl;

    """]

    [[completions]]
    completion_label = "DebugImpl::fmt(...)"
    completion_label_type_info = "fn(self: @T, ref f: Formatter) -> Result<(), Error>"
    insert_text = "DebugImpl::fmt(${1:f})"
    text_edits = ["""
    use core::fmt::into_felt252_based::DebugImpl;

    """]

    [[completions]]
    completion_label = "EcPointImpl::y(...)"
    completion_label_type_info = "fn(self: NonZero<EcPoint>) -> felt252"
    insert_text = "EcPointImpl::y()"
    text_edits = ["""
    use core::ec::EcPointImpl;

    """]

    [[completions]]
    completion_label = "EcPointTrait::y(...)"
    completion_label_type_info = "fn(self: NonZero<EcPoint>) -> felt252"
    insert_text = "EcPointTrait::y()"
    text_edits = ["""
    use core::ec::EcPointTrait;

    """]

    [[completions]]
    completion_label = "HashImpl"
    completion_label_path = "(use core::hash::into_felt252_based::HashImpl)"
    text_edits = ["""
    use core::hash::into_felt252_based::HashImpl;

    """]

    [[completions]]
    completion_label = "HashImpl::update_state(...)"
    completion_label_type_info = "fn(state: S, value: T) -> S"
    insert_text = "HashImpl::update_state(${1:state}, ${2:value})"
    text_edits = ["""
    use core::hash::into_felt252_based::HashImpl;

    """]

    [[completions]]
    completion_label = "Map {...}"
    completion_label_path = "(use starknet::storage::Map)"
    completion_label_type_info = "Map {}"
    insert_text = "Map {}"
    text_edits = ["""
    use starknet::storage::Map;

    """]

    [[completions]]
    completion_label = "SerdeImpl"
    completion_label_path = "(use core::serde::into_felt252_based::SerdeImpl)"
    text_edits = ["""
    use core::serde::into_felt252_based::SerdeImpl;

    """]

    [[completions]]
    completion_label = "SerdeImpl::deserialize(...)"
    completion_label_type_info = "fn(ref serialized: Span<felt252>) -> Option<T>"
    insert_text = "SerdeImpl::deserialize(${1:serialized})"
    text_edits = ["""
    use core::serde::into_felt252_based::SerdeImpl;

    """]

    [[completions]]
    completion_label = "SerdeImpl::serialize(...)"
    completion_label_type_info = "fn(self: @T, ref output: Array<felt252>) -> ()"
    insert_text = "SerdeImpl::serialize(${1:output})"
    text_edits = ["""
    use core::serde::into_felt252_based::SerdeImpl;

    """]
    "#);
}

#[test]
fn trait_item_suffix_by_trait_name() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait UniqueXyzTrait {
        fn unique_xyz_func() -> u32;
        type UniqueXyzType;
        const UNIQUE_XYZ_CONST: u32;
    }

    fn test() {
        UniqueXyz<caret>
    }
    ",@r#"
    caret = """
        UniqueXyz<caret>
    """

    [[completions]]
    completion_label = "UniqueXyzTrait"

    [[completions]]
    completion_label = "UniqueXyzTrait::UNIQUE_XYZ_CONST"
    completion_label_type_info = "u32"

    [[completions]]
    completion_label = "UniqueXyzTrait::UniqueXyzType"

    [[completions]]
    completion_label = "UniqueXyzTrait::unique_xyz_func(...)"
    completion_label_type_info = "fn() -> u32"
    insert_text = "UniqueXyzTrait::unique_xyz_func()"

    [[completions]]
    completion_label = "UnitInt"
    completion_label_path = "(use core::internal::bounded_int::UnitInt)"
    text_edits = ["""
    use core::internal::bounded_int::UnitInt;

    """]
    "#);
}

#[test]
fn trait_item_suffix_by_item_name() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait UniqueXyzTrait {
        fn unique_xyz_func() -> u32;
    }

    fn test() {
        unique_xyz<caret>
    }
    ",@r#"
    caret = """
        unique_xyz<caret>
    """

    [[completions]]
    completion_label = "UniqueXyzTrait::unique_xyz_func(...)"
    completion_label_type_info = "fn() -> u32"
    insert_text = "UniqueXyzTrait::unique_xyz_func()"

    [[completions]]
    completion_label = "BoxTrait::unbox(...)"
    completion_label_type_info = "fn(self: Box<T>) -> T nopanic"
    insert_text = "BoxTrait::unbox()"

    [[completions]]
    completion_label = "PartialEq::ne(...)"
    completion_label_type_info = "fn(lhs: @T, rhs: @T) -> bool"
    insert_text = "PartialEq::ne(${1:lhs}, ${2:rhs})"

    [[completions]]
    completion_label = "BoxImpl::unbox(...)"
    completion_label_type_info = "fn(self: Box<T>) -> T nopanic"
    insert_text = "BoxImpl::unbox()"
    text_edits = ["""
    use core::box::BoxImpl;

    """]
    "#);
}

#[test]
fn trait_item_suffix_from_other_module() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod my_mod {
        pub trait UniqueXyzTrait {
            fn unique_xyz_func() -> u32;
        }
    }

    fn test() {
        UniqueXyz<caret>
    }
    ",@r#"
    caret = """
        UniqueXyz<caret>
    """

    [[completions]]
    completion_label = "UniqueXyzTrait"
    completion_label_path = "(use my_mod::UniqueXyzTrait)"
    text_edits = ["""
    use my_mod::UniqueXyzTrait;

    """]

    [[completions]]
    completion_label = "UniqueXyzTrait::unique_xyz_func(...)"
    completion_label_type_info = "fn() -> u32"
    insert_text = "UniqueXyzTrait::unique_xyz_func()"
    text_edits = ["""
    use my_mod::UniqueXyzTrait;

    """]

    [[completions]]
    completion_label = "UnitInt"
    completion_label_path = "(use core::internal::bounded_int::UnitInt)"
    text_edits = ["""
    use core::internal::bounded_int::UnitInt;

    """]
    "#);
}
