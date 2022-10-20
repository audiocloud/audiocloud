use schemars::schema::RootSchema;
use serde_json::{json, Value};
use utoipa::openapi::OpenApi;

pub use codec::*;

pub mod codec;

pub fn merge_schemas(x: impl Iterator<Item = RootSchema>) -> RootSchema {
    let mut root = RootSchema::default();
    for schema in x {
        let RootSchema {
            schema,
            definitions,
            ..
        } = schema;
        let title = schema.metadata.as_ref().unwrap().title.clone().unwrap();

        let title = if title.starts_with("Array_of_") {
            format!("{}List", &title[9..])
        } else {
            title
        };

        root.definitions.extend(definitions.into_iter());
        root.definitions.insert(title, schema.into());
    }

    root
}

pub fn openapi_with_schemas_to_json(
    api: OpenApi,
    merged: RootSchema,
    patch: Value,
) -> anyhow::Result<String> {
    let mut api: serde_json::Value = serde_json::from_str(&api.to_json()?)?;

    let schemas = serde_json::to_value(&merged.definitions)?;

    api.as_object_mut().expect("as object").insert(
        "components".to_string(),
        json!({
            "schemas": schemas,
        }),
    );

    let patches: Vec<jatch::Patch> = serde_json::from_value(patch)?;
    let api = jatch::apply(api, patches)?;

    Ok(serde_json::to_string_pretty(&api)?.replace("#/definitions/", "#/components/schemas/"))
}

pub fn openapi_set_version(version: &str) -> serde_json::Value {
    json!({
        "op": "replace",
        "path": "/openapi",
        "value": version
    })
}

pub fn openapi_set_info_title(title: &str) -> serde_json::Value {
    json!({
        "op": "replace",
        "path": "/info/title",
        "value": title
    })
}

pub fn openapi_add_apache_license() -> serde_json::Value {
    json!({
    "op": "replace",
    "path": "/info/license",
    "value": json!({
        "name": "Apache 2.0",
        "url": "https://www.apache.org/licenses/LICENSE-2.0.html"
    })})
}

pub fn openapi_create_empty_servers() -> serde_json::Value {
    json!({
        "op": "replace",
        "path": "/servers",
        "value": []
    })
}

pub fn openapi_add_server(url: &str, description: &str) -> serde_json::Value {
    json!({
    "op": "add",
    "path": "/servers/-",
    "value": {
        "description": description,
        "url": url
    }})
}
