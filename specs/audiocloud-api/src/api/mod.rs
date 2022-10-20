use schemars::schema::RootSchema;

pub use codec::*;

pub mod codec;

pub fn merge_schemas(x: impl Iterator<Item=RootSchema>) -> RootSchema {
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
