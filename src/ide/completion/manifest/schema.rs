//! Thin wrapper around `taplo-common`'s schema traversal for TOML completion.
//!
//! All `$ref`s in the Scarb manifest schema are local (`#/$defs/…`), so
//! no HTTP requests are ever made; `block_on` is used to drive the async API
//! from the synchronous completion handler.

use std::sync::{Arc, OnceLock};

use lsp_types::Url;
use scarb_manifest_schema::SCARB_SCHEMA_JSON;
use serde_json::Value;
use taplo::dom::Keys;
use taplo_common::environment::native::NativeEnvironment;
use taplo_common::schema::Schemas;

const SCARB_SCHEMA_URL: &str = "schema://scarb-manifest";

fn rt() -> &'static tokio::runtime::Runtime {
    static RT: OnceLock<tokio::runtime::Runtime> = OnceLock::new();
    RT.get_or_init(|| {
        tokio::runtime::Builder::new_current_thread()
            .build()
            .expect("failed to build tokio runtime for schema traversal")
    })
}

fn schemas() -> &'static Schemas<NativeEnvironment> {
    static SCHEMAS: OnceLock<Schemas<NativeEnvironment>> = OnceLock::new();
    SCHEMAS.get_or_init(|| {
        rt().block_on(async {
            let env = NativeEnvironment::new();
            let schemas = Schemas::new(env, reqwest::Client::new());
            let url = Url::parse(SCARB_SCHEMA_URL).unwrap();
            let schema: Value =
                serde_json::from_str(SCARB_SCHEMA_JSON).expect("invalid schema JSON");
            schemas.add_schema(&url, Arc::new(schema)).await;
            schemas
        })
    })
}

fn schema_url() -> &'static Url {
    static URL: OnceLock<Url> = OnceLock::new();
    URL.get_or_init(|| Url::parse(SCARB_SCHEMA_URL).unwrap())
}

fn scarb_schema_arc() -> &'static Arc<Value> {
    static SCHEMA: OnceLock<Arc<Value>> = OnceLock::new();
    SCHEMA.get_or_init(|| {
        Arc::new(serde_json::from_str(SCARB_SCHEMA_JSON).expect("invalid schema JSON"))
    })
}

/// Collects possible child schemas from a given path in the schema,
/// delegating to `taplo_common::schema::Schemas::possible_schemas_from`.
///
/// `taplo-common`'s LRU cache expires every 60 s and clears the in-memory
/// schema. Since our `schema://` URL is not re-fetchable via HTTP or file,
/// we catch the resulting error and re-register the embedded schema before
/// retrying once (the expiry only fires once per window, so the retry succeeds).
pub fn possible_schemas_from(
    value: &Value,
    path: &Keys,
    max_depth: usize,
) -> Vec<(Keys, Keys, Arc<Value>)> {
    let url = schema_url();
    let s = schemas();
    rt().block_on(async {
        match s.possible_schemas_from(url, value, path, max_depth).await {
            Ok(r) => r,
            Err(_) => {
                s.add_schema(url, scarb_schema_arc().clone()).await;
                s.possible_schemas_from(url, value, path, max_depth).await.unwrap_or_default()
            }
        }
    })
}

/// Check if a value is a `$ref`.
pub fn is_schema_ref(schema: &Value) -> bool {
    schema.get("$ref").and_then(Value::as_str).is_some()
}

/// Get the documentation string from a schema node.
pub fn schema_documentation(schema: &Value) -> Option<String> {
    schema["description"].as_str().map(Into::into)
}
