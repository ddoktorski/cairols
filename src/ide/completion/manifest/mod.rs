mod schema;

use std::fmt::Write as _;

use lsp_types::{
    CompletionItem, CompletionItemKind, CompletionParams, CompletionResponse, CompletionTextEdit,
    Documentation, InsertTextFormat, MarkupContent, MarkupKind, TextEdit, Url,
};
use serde_json::Value;
use taplo::dom::{Keys, Node, node::TableKind};
use taplo::parser::parse as taplo_parse;
use taplo_lsp::query::{Query, lookup_keys};

use self::schema::{is_schema_ref, possible_schemas_from, schema_documentation};

/// Default max dotted-key depth for completions.
const MAX_KEYS: usize = 2;

/// Checks if the given URI points to a `Scarb.toml` manifest file.
pub fn is_scarb_manifest(uri: &Url) -> bool {
    uri.path().ends_with("Scarb.toml")
}

/// Compute completion items for a `Scarb.toml` manifest file.
pub fn manifest_completions(params: CompletionParams, content: &str) -> Option<CompletionResponse> {
    let parsed = taplo_parse(content);
    let dom = parsed.into_dom();
    let mapper = lsp_async_stub::util::Mapper::new_utf16(content, false);

    let position = params.text_document_position.position;
    let offset = mapper.offset(lsp_async_stub::util::Position::new(
        position.line as u64,
        position.character as u64,
    ))?;

    let query = Query::at(&dom, offset);

    let value = serde_json::to_value(&dom).unwrap_or(Value::Null);

    if query.in_table_header() {
        return complete_table_header(&query, &dom, &value, &mapper);
    }

    if query.in_table_array_header() {
        return complete_table_array_header(&query, &dom, &value, &mapper);
    }

    if query.empty_line() {
        return complete_empty_line(&query, &dom, &value, &mapper);
    }

    if query.in_entry_keys() {
        return complete_entry_keys(&query, &dom, &value, &mapper);
    }

    if query.in_entry_value() {
        return complete_entry_value(&query, &dom, &value, &mapper);
    }

    // Standalone partial keys (fallback).
    complete_standalone_keys(&query, &dom, &value, &mapper)
}

fn complete_table_header(
    query: &Query,
    dom: &Node,
    value: &Value,
    mapper: &lsp_async_stub::util::Mapper,
) -> Option<CompletionResponse> {
    let key_count = query.header_keys().len();
    let object_schemas = possible_schemas_from(value, &Keys::empty(), key_count + MAX_KEYS + 1);

    let key_range = query
        .header_key()
        .map(|k| k.text_range())
        .and_then(|r| if r.is_empty() { None } else { Some(r) });

    let node = query.dom_node().cloned().unwrap_or_else(|| (Keys::empty(), dom.clone()));

    let items = object_schemas
        .into_iter()
        .filter(|(_, _, s)| {
            s["type"].is_null()
                || s["type"] == "object"
                || s["type"].as_array().is_some_and(|arr| arr.iter().any(|v| v == "object"))
        })
        .filter(|(full_key, _, _)| match dom.path(full_key) {
            Some(n) => {
                node.0 == *full_key || n.as_table().is_some_and(|t| t.kind() == TableKind::Pseudo)
            }
            None => true,
        })
        .map(|(full_key, _, s)| CompletionItem {
            label: full_key.to_string(),
            kind: Some(CompletionItemKind::STRUCT),
            documentation: documentation(&s),
            text_edit: key_range.map(|r| {
                CompletionTextEdit::Edit(TextEdit {
                    range: mapper_range_to_lsp(mapper, r),
                    new_text: full_key.to_string(),
                })
            }),
            ..Default::default()
        })
        .collect();

    Some(CompletionResponse::Array(items))
}

fn complete_table_array_header(
    query: &Query,
    _dom: &Node,
    value: &Value,
    mapper: &lsp_async_stub::util::Mapper,
) -> Option<CompletionResponse> {
    let key_count = query.header_keys().len();
    let schemas = possible_schemas_from(value, &Keys::empty(), key_count + MAX_KEYS + 1);

    let key_range = query
        .header_key()
        .map(|k| k.text_range())
        .and_then(|r| if r.is_empty() { None } else { Some(r) });

    let items = schemas
        .into_iter()
        .filter(|(_, _, s)| {
            s["type"] == "array" && (s["items"]["type"] == "object" || s["items"]["type"].is_null())
        })
        .map(|(full_key, _, s)| CompletionItem {
            label: full_key.to_string(),
            kind: Some(CompletionItemKind::STRUCT),
            documentation: documentation(&s),
            text_edit: key_range.map(|r| {
                CompletionTextEdit::Edit(TextEdit {
                    range: mapper_range_to_lsp(mapper, r),
                    new_text: full_key.to_string(),
                })
            }),
            ..Default::default()
        })
        .collect();

    Some(CompletionResponse::Array(items))
}

fn complete_empty_line(
    query: &Query,
    dom: &Node,
    value: &Value,
    _mapper: &lsp_async_stub::util::Mapper,
) -> Option<CompletionResponse> {
    let parent_table = query.parent_table_or_array_table(dom);

    let schemas =
        possible_schemas_from(value, &lookup_keys(dom.clone(), &parent_table.0), MAX_KEYS + 1);

    let items = schemas
        .into_iter()
        .filter(|(full_key, _, _)| match dom.path(full_key) {
            Some(n) => n.as_table().is_some_and(|t| t.kind() == TableKind::Pseudo),
            None => true,
        })
        .map(|(_, relative_keys, schema)| CompletionItem {
            label: relative_keys.to_string(),
            kind: Some(CompletionItemKind::VARIABLE),
            documentation: documentation(&schema),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text: Some(new_entry_snippet(&relative_keys, &schema)),
            ..Default::default()
        })
        .collect();

    Some(CompletionResponse::Array(items))
}

fn complete_entry_keys(
    query: &Query,
    dom: &Node,
    value: &Value,
    mapper: &lsp_async_stub::util::Mapper,
) -> Option<CompletionResponse> {
    let mut parent_keys = if let Some((k, _)) = query.dom_node() {
        k.clone()
    } else {
        query.parent_table_or_array_table(dom).0
    };

    let entry_keys = query.entry_keys();
    parent_keys = parent_keys.skip_right(entry_keys.len());

    let schemas = possible_schemas_from(
        value,
        &lookup_keys(dom.clone(), &parent_keys),
        entry_keys.len() + MAX_KEYS + 1,
    );

    let key_range = query.entry_key().map(|k| k.text_range());
    let has_eq = query.entry_has_eq();

    let items = schemas
        .into_iter()
        .map(|(_, relative_keys, schema)| {
            let text = if has_eq {
                relative_keys.to_string() + " "
            } else {
                new_entry_snippet(&relative_keys, &schema)
            };
            CompletionItem {
                label: relative_keys.to_string(),
                kind: Some(CompletionItemKind::VARIABLE),
                documentation: documentation(&schema),
                text_edit: key_range.map(|r| {
                    CompletionTextEdit::Edit(TextEdit {
                        range: mapper_range_to_lsp(mapper, r),
                        new_text: text.clone(),
                    })
                }),
                insert_text: Some(text),
                insert_text_format: if has_eq { None } else { Some(InsertTextFormat::SNIPPET) },
                ..Default::default()
            }
        })
        .collect();

    Some(CompletionResponse::Array(items))
}

fn complete_entry_value(
    query: &Query,
    dom: &Node,
    value: &Value,
    mapper: &lsp_async_stub::util::Mapper,
) -> Option<CompletionResponse> {
    let (path, _) = query.dom_node()?;

    // Inline table: same as empty line but within the table.
    if query.in_inline_table() {
        let schemas = possible_schemas_from(value, &lookup_keys(dom.clone(), path), MAX_KEYS + 1);

        let items = schemas
            .into_iter()
            .filter(|(full_key, _, _)| match dom.path(full_key) {
                Some(n) => n.as_table().is_some_and(|t| t.kind() == TableKind::Pseudo),
                None => true,
            })
            .map(|(_, relative_keys, schema)| CompletionItem {
                label: relative_keys.to_string(),
                kind: Some(CompletionItemKind::VARIABLE),
                documentation: documentation(&schema),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                insert_text: Some(new_entry_snippet(&relative_keys, &schema)),
                ..Default::default()
            })
            .collect();

        return Some(CompletionResponse::Array(items));
    }

    let path = if query.is_inline() {
        lookup_keys(dom.clone(), &path.clone())
    } else {
        let parent = query.parent_table_or_array_table(dom);
        let entry_key = query.entry_keys();
        lookup_keys(dom.clone(), &parent.0.extend(entry_key))
    };

    let schemas = possible_schemas_from(value, &path, MAX_KEYS + 1);

    let range = if query.in_array() {
        None
    } else {
        query.entry_value().map(|k| k.text_range()).map(|r| mapper_range_to_lsp(mapper, r))
    };

    let single_quote = query.is_single_quote_value();
    let mut completions = Vec::new();

    for (_, _, schema) in schemas {
        add_value_completions(&schema, range, &mut completions, single_quote);
    }

    Some(CompletionResponse::Array(completions))
}

fn complete_standalone_keys(
    query: &Query,
    dom: &Node,
    value: &Value,
    mapper: &lsp_async_stub::util::Mapper,
) -> Option<CompletionResponse> {
    let mut parent_keys = if let Some((k, _)) = query.dom_node() {
        k.clone()
    } else {
        query.parent_table_or_array_table(dom).0
    };

    let entry_keys = query.entry_keys();
    parent_keys = parent_keys.skip_right(entry_keys.len());

    let schemas =
        possible_schemas_from(value, &lookup_keys(dom.clone(), &parent_keys), MAX_KEYS + 1);

    if entry_keys.is_empty() {
        return None;
    }

    let items = schemas
        .into_iter()
        .filter(|(full_key, _, _)| match dom.path(full_key) {
            Some(n) => n.as_table().is_some_and(|t| t.kind() == TableKind::Pseudo),
            None => true,
        })
        .map(|(_, relative_keys, schema)| CompletionItem {
            label: relative_keys.to_string(),
            kind: Some(CompletionItemKind::VARIABLE),
            documentation: documentation(&schema),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                range: mapper_range_to_lsp(mapper, entry_keys.all_text_range()),
                new_text: new_entry_snippet(&relative_keys, &schema),
            })),
            ..Default::default()
        })
        .collect();

    Some(CompletionResponse::Array(items))
}

fn documentation(schema: &Value) -> Option<Documentation> {
    let docs = schema_documentation(schema)?;
    Some(Documentation::MarkupContent(MarkupContent { kind: MarkupKind::Markdown, value: docs }))
}

fn add_value_completions(
    schema: &Value,
    range: Option<lsp_types::Range>,
    completions: &mut Vec<CompletionItem>,
    single_quote: bool,
) {
    let schema_docs = schema["description"].as_str().map(String::from);

    if let Some(enum_values) = schema["enum"].as_array() {
        for (idx, val) in enum_values.iter().enumerate() {
            let Ok(node): Result<Node, _> = serde_json::from_value(val.clone()) else {
                continue;
            };
            let toml_value = node.to_toml(true, single_quote);
            completions.push(CompletionItem {
                label: toml_value.clone(),
                sort_text: Some(format!("{idx}{toml_value}")),
                kind: Some(match node {
                    Node::Table(_) => CompletionItemKind::STRUCT,
                    _ => CompletionItemKind::VALUE,
                }),
                documentation: schema_docs.clone().map(|value| {
                    Documentation::MarkupContent(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value,
                    })
                }),
                text_edit: range.map(|range| {
                    CompletionTextEdit::Edit(TextEdit { range, new_text: toml_value })
                }),
                ..Default::default()
            });
        }
        return;
    }

    if let Some(const_value) = schema.get("const")
        && !const_value.is_null()
        && let Ok(node) = serde_json::from_value::<Node>(const_value.clone())
    {
        let toml_value = node.to_toml(true, single_quote);
        completions.push(CompletionItem {
            label: toml_value.clone(),
            kind: Some(match node {
                Node::Table(_) => CompletionItemKind::STRUCT,
                _ => CompletionItemKind::VALUE,
            }),
            documentation: schema_docs.clone().map(|value| {
                Documentation::MarkupContent(MarkupContent { kind: MarkupKind::Markdown, value })
            }),
            text_edit: range
                .map(|range| CompletionTextEdit::Edit(TextEdit { range, new_text: toml_value })),
            ..Default::default()
        });
    }

    if let Some(default_value) = schema.get("default")
        && !default_value.is_null()
        && let Ok(node) = serde_json::from_value::<Node>(default_value.clone())
    {
        let toml_value = node.to_toml(true, single_quote);
        completions.push(CompletionItem {
            label: toml_value.clone(),
            kind: Some(match node {
                Node::Table(_) => CompletionItemKind::STRUCT,
                _ => CompletionItemKind::VALUE,
            }),
            documentation: schema_docs.clone().map(|value| {
                Documentation::MarkupContent(MarkupContent { kind: MarkupKind::Markdown, value })
            }),
            text_edit: range
                .map(|range| CompletionTextEdit::Edit(TextEdit { range, new_text: toml_value })),
            ..Default::default()
        });
    }

    let types = match schema["type"].clone() {
        Value::Null => vec![Value::String("object".into())],
        Value::String(s) => vec![Value::String(s)],
        Value::Array(tys) => tys,
        _ => Vec::new(),
    };

    for ty in types {
        if let Some(s) = ty.as_str() {
            match s {
                "string" => {
                    completions.push(CompletionItem {
                        label: r#""""#.into(),
                        kind: Some(CompletionItemKind::VALUE),
                        documentation: Some(Documentation::MarkupContent(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: schema_docs.clone().unwrap_or_else(|| "string".into()),
                        })),
                        insert_text_format: Some(InsertTextFormat::SNIPPET),
                        text_edit: range.map(|range| {
                            CompletionTextEdit::Edit(TextEdit { range, new_text: r#""$0""#.into() })
                        }),
                        ..Default::default()
                    });
                }
                "boolean" => {
                    completions.push(CompletionItem {
                        label: "true".into(),
                        kind: Some(CompletionItemKind::VALUE),
                        documentation: Some(Documentation::MarkupContent(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: schema_docs.clone().unwrap_or_else(|| "true value".into()),
                        })),
                        insert_text_format: Some(InsertTextFormat::SNIPPET),
                        text_edit: range.map(|range| {
                            CompletionTextEdit::Edit(TextEdit { range, new_text: "true$0".into() })
                        }),
                        ..Default::default()
                    });
                    completions.push(CompletionItem {
                        label: "false".into(),
                        kind: Some(CompletionItemKind::VALUE),
                        documentation: Some(Documentation::MarkupContent(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: schema_docs.clone().unwrap_or_else(|| "false value".into()),
                        })),
                        insert_text_format: Some(InsertTextFormat::SNIPPET),
                        text_edit: range.map(|range| {
                            CompletionTextEdit::Edit(TextEdit { range, new_text: "false$0".into() })
                        }),
                        ..Default::default()
                    });
                }
                "array" => {
                    completions.push(CompletionItem {
                        label: "[]".into(),
                        kind: Some(CompletionItemKind::VALUE),
                        documentation: Some(Documentation::MarkupContent(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: schema_docs.clone().unwrap_or_else(|| "array".into()),
                        })),
                        insert_text_format: Some(InsertTextFormat::SNIPPET),
                        text_edit: range.map(|range| {
                            CompletionTextEdit::Edit(TextEdit { range, new_text: "[$0]".into() })
                        }),
                        ..Default::default()
                    });
                }
                "object" => {
                    completions.push(CompletionItem {
                        label: "{ }".into(),
                        kind: Some(CompletionItemKind::VALUE),
                        documentation: Some(Documentation::MarkupContent(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: schema_docs.clone().unwrap_or_else(|| "object".into()),
                        })),
                        insert_text_format: Some(InsertTextFormat::SNIPPET),
                        text_edit: range.map(|range| {
                            CompletionTextEdit::Edit(TextEdit { range, new_text: "{ $0 }".into() })
                        }),
                        ..Default::default()
                    });
                }
                _ => {}
            }
        }
    }
}

fn new_entry_snippet(keys: &Keys, schema: &Value) -> String {
    let value = default_value_snippet(schema, 0);
    format!("{keys} = {value}")
}

fn default_value_snippet(schema: &Value, cursor_count: usize) -> String {
    if let Some(const_value) = schema.get("const")
        && !const_value.is_null()
        && let Ok(node) = serde_json::from_value::<Node>(const_value.clone())
    {
        return format!("${{{}:{}}}", cursor_count, node.to_toml(true, false));
    }

    if let Some(default_value) = schema.get("default")
        && !default_value.is_null()
        && let Ok(node) = serde_json::from_value::<Node>(default_value.clone())
    {
        return format!("${{{}:{}}}", cursor_count, node.to_toml(true, false));
    }

    if schema.get("enum").is_some() {
        return format!("${cursor_count}");
    }

    let mut init_keys = Vec::new();

    if let Some(arr) = schema["required"].as_array() {
        init_keys.extend(arr.iter().filter_map(|s| s.as_str().map(ToString::to_string)));
    }

    init_keys.dedup();

    if !init_keys.is_empty() {
        let mut s = String::new();
        s += "{ ";
        for (i, init_key) in init_keys.iter().enumerate() {
            if i != 0 {
                s += ", ";
            }
            write!(
                s,
                "{init_key} = {}",
                default_value_snippet(&schema["properties"][init_key.as_str()], cursor_count + 1)
            )
            .unwrap();
        }
        s += " }$0";
        return s;
    }

    empty_value_snippet(schema, cursor_count)
}

fn empty_value_snippet(schema: &Value, cursor_count: usize) -> String {
    if is_schema_ref(schema) {
        return format!("${cursor_count}");
    }

    match &schema["type"] {
        Value::Null => format!("{{ ${cursor_count} }}"),
        Value::String(s) => match s.as_str() {
            "object" => format!("{{ ${cursor_count} }}"),
            "array" => format!("[${cursor_count}]"),
            "string" => format!(r#""${cursor_count}""#),
            "boolean" => format!("${{{cursor_count}:false}}"),
            _ => format!("${cursor_count}"),
        },
        _ => format!("${cursor_count}"),
    }
}

/// Convert a `taplo::rowan::TextRange` to an `lsp_types::Range` via the mapper.
fn mapper_range_to_lsp(
    mapper: &lsp_async_stub::util::Mapper,
    range: taplo::rowan::TextRange,
) -> lsp_types::Range {
    let r = mapper.range(range).expect("range should be valid");
    lsp_types::Range {
        start: lsp_types::Position {
            line: r.start.line as u32,
            character: r.start.character as u32,
        },
        end: lsp_types::Position { line: r.end.line as u32, character: r.end.character as u32 },
    }
}
