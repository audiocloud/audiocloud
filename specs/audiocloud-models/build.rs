use std::collections::HashMap;
use std::fs::File;
use std::{env, fs};

use askama::Template;
use itertools::Itertools;

use audiocloud_api::*;

#[derive(Template)]
#[template(path = "rust_preset.rs", escape = "none")]
struct RustPresetModelTemplate<'a> {
    rust_name: String,
    model: &'a Model,
}

impl<'a> RustPresetModelTemplate<'a> {
    pub fn new(name: &str, model: &'a Model) -> Self {
        Self {
            rust_name: pascal_case_converter().convert(name),
            model,
        }
    }
}

#[derive(Template)]
#[template(path = "rust_parameters.rs", escape = "none")]
struct RustParamsModelTemplate<'a> {
    rust_name: String,
    model: &'a Model,
}

impl<'a> RustParamsModelTemplate<'a> {
    pub fn new(name: &str, model: &'a Model) -> Self {
        Self {
            rust_name: pascal_case_converter().convert(name),
            model,
        }
    }
}

#[derive(Template)]
#[template(path = "rust_reports.rs", escape = "none")]
struct RustReportsModelTemplate<'a> {
    rust_name: String,
    model: &'a Model,
}

impl<'a> RustReportsModelTemplate<'a> {
    pub fn new(name: &str, model: &'a Model) -> Self {
        Self {
            rust_name: pascal_case_converter().convert(name),
            model,
        }
    }
}

#[derive(Template)]
#[template(path = "rust_consts.rs", escape = "none")]
struct RustConstantsTemplate<'a> {
    model: &'a Model,
}

#[derive(Template)]
#[template(path = "rust_model_values.rs", escape = "none")]
struct ModelValueOptionsTemplate<'a> {
    values: &'a Vec<ModelValueOption>,
}

impl<'a> ModelValueOptionsTemplate<'a> {
    pub fn new(values: &'a Vec<ModelValueOption>) -> Self {
        Self { values }
    }
}

#[derive(Template)]
#[template(path = "rust_model_value.rs", escape = "none")]
struct ModelValueTemplate<'a> {
    value: &'a ModelValue,
}

impl<'a> ModelValueTemplate<'a> {
    pub fn new(value: &'a ModelValue) -> Self {
        Self { value }
    }
}

impl<'a> RustConstantsTemplate<'a> {
    pub fn new(model: &'a Model) -> Self {
        Self { model }
    }
}

#[derive(Template)]
#[template(path = "ts_preset.ts", escape = "none")]
struct TSPresetModelTemplate<'a> {
    ts_name: String,
    model: &'a Model,
}

impl<'a> TSPresetModelTemplate<'a> {
    pub fn new(name: &str, model: &'a Model) -> Self {
        Self {
            ts_name: pascal_case_converter().convert(name),
            model,
        }
    }
}

#[derive(Template)]
#[template(path = "ts_parameters.ts", escape = "none")]
struct TSParamsModelTemplate<'a> {
    ts_name: String,
    model: &'a Model,
}

impl<'a> TSParamsModelTemplate<'a> {
    pub fn new(name: &str, model: &'a Model) -> Self {
        Self {
            ts_name: pascal_case_converter().convert(name),
            model,
        }
    }
}

#[derive(Template)]
#[template(path = "ts_reports.ts", escape = "none")]
struct TSReportsModelTemplate<'a> {
    ts_name: String,
    model: &'a Model,
}

impl<'a> TSReportsModelTemplate<'a> {
    pub fn new(name: &str, model: &'a Model) -> Self {
        Self {
            ts_name: pascal_case_converter().convert(name),
            model,
        }
    }
}

#[derive(Template)]
#[template(path = "rust_generated.rs", escape = "none")]
struct RustGeneratedTemplate<'a> {
    models: &'a HashMap<String, HashMap<String, Model>>,
}

#[derive(Template)]
#[template(path = "ts_generated.ts", escape = "none")]
struct TSGeneratedTemplate<'a> {
    models: &'a HashMap<String, HashMap<String, Model>>,
}

mod filters {
    use audiocloud_api::{Model, ModelParameter, ModelReport};

    use crate::{pascal_case_converter, rust_type, screaming_snake_case_converter, ts_type};

    pub fn rust_preset_type(spec: &(&ModelParameter, &Model)) -> ::askama::Result<String> {
        Ok(rust_type(&spec.0.values, spec.0.scope, spec.1, true))
    }

    pub fn rust_param_type(spec: &(&ModelParameter, &Model)) -> ::askama::Result<String> {
        Ok(rust_type(&spec.0.values, spec.0.scope, spec.1, false))
    }

    pub fn rust_report_type(spec: &(&ModelReport, &Model)) -> askama::Result<String> {
        Ok(rust_type(&spec.0.values, spec.0.scope, spec.1, false))
    }

    pub fn ts_preset_type(spec: &(&ModelParameter, &Model)) -> ::askama::Result<String> {
        Ok(ts_type(&spec.0.values, spec.0.scope, spec.1, true))
    }

    pub fn ts_param_type(spec: &(&ModelParameter, &Model)) -> ::askama::Result<String> {
        Ok(ts_type(&spec.0.values, spec.0.scope, spec.1, false))
    }

    pub fn ts_report_type(spec: &(&ModelReport, &Model)) -> ::askama::Result<String> {
        Ok(ts_type(&spec.0.values, spec.0.scope, spec.1, false))
    }

    pub fn pascal_case(value: &String) -> ::askama::Result<String> {
        Ok(pascal_case_converter().convert(value))
    }

    pub fn screaming_snake(value: &String) -> ::askama::Result<String> {
        Ok(screaming_snake_case_converter().convert(value))
    }
}

fn main() {
    // read env variable MODELS_PATH and deafualt
    let models_dir = env::var("MODELS_DIR").unwrap_or_else(|_| "models".to_owned());
    let mut by_manufacturers = HashMap::<String, HashMap<String, Model>>::new();
    for model_path in globwalk::GlobWalkerBuilder::from_patterns(models_dir, &["*.yaml", "*.yml"])
        .max_depth(4)
        .follow_links(true)
        .build()
        .expect("create globber")
        .into_iter()
        .filter_map(Result::ok)
    {
        let model_path = model_path.path();
        let model_file_stem = model_path
            .file_stem()
            .expect("get file name")
            .to_string_lossy()
            .to_string();
        let (manufacturer, name) =
            model_file_stem.split_at(model_file_stem.find('_').expect("file name must have '_'"));
        let name = &name[1..];

        println!("cargo:rerun-if-changed={model_path:?}");
        if let Ok(model_file) = File::open(model_path) {
            match serde_yaml::from_reader::<_, Model>(model_file) {
                Ok(model_content) => {
                    // create rust types
                    by_manufacturers
                        .entry(manufacturer.to_owned())
                        .or_default()
                        .insert(name.to_owned(), model_content);
                }
                Err(err) => {
                    eprintln!("Failed to parse {model_path:?}: {err} ({err:?})");
                }
            }
        }
    }

    fs::write(
        "src/generated.rs",
        RustGeneratedTemplate {
            models: &by_manufacturers,
        }
        .render()
        .expect("render rust types"),
    )
    .expect("write generated rust code");

    let _ = std::process::Command::new("cargo")
        .arg("+nightly")
        .arg("fmt")
        .arg("--")
        .arg("src/generated.rs")
        .output();

    // fs::write("../packages/models/src/generated.ts", TSGeneratedTemplate { models: &by_manufacturers }.render()
    //               .expect("render typescript types")).expect("write generated typescript code");
}

fn simple_to_rust_type(simple_type: SimpleModelValueType) -> &'static str {
    match simple_type {
        SimpleModelValueType::String => "String",
        SimpleModelValueType::Number { integer, signed } => {
            if integer {
                if signed {
                    "i64"
                } else {
                    "u64"
                }
            } else {
                "f64"
            }
        }
        SimpleModelValueType::Bool => "bool",
    }
}

fn simple_to_ts_type(simple_type: SimpleModelValueType) -> &'static str {
    match simple_type {
        SimpleModelValueType::String => "string",
        SimpleModelValueType::Number { .. } => "number",
        SimpleModelValueType::Bool => "boolean",
    }
}

fn ts_type(
    options: &Vec<ModelValueOption>,
    scope: ModelElementScope,
    model: &Model,
    _preset: bool,
) -> String {
    let inner_type = match get_values_type(options).expect("get params types") {
        ModelValueType::Single(s) => simple_to_ts_type(s).to_owned(),
        ModelValueType::Either(a, b) => {
            if a.is_bool() {
                format!("T.ToggleOr<{}>", simple_to_ts_type(b))
            } else if b.is_bool() {
                format!("T.ToggleOr<{}>", simple_to_ts_type(a))
            } else {
                format!(
                    "T.Either<{}, {}>",
                    simple_to_ts_type(a),
                    simple_to_rust_type(b)
                )
            }
        }
        ModelValueType::Any => "any".to_owned(),
    };

    match scope {
        ModelElementScope::Global => format!("Array<{inner_type}>"),
        ModelElementScope::AllInputs => match model.inputs.len() {
            0 => return "null".to_owned(),
            1 => return inner_type,
            2 => format!("T.Stereo<{inner_type}>"),
            i => format!("T.Tuple{i}<{inner_type}>"),
        },
        ModelElementScope::AllOutputs => match model.outputs.len() {
            0 => return "null".to_owned(),
            1 => return inner_type,
            2 => format!("T.Stereo<{inner_type}>"),
            i => format!("T.Tuple{i}<{inner_type}>"),
        },
        ModelElementScope::Count(i) => format!("T.Tuple{i}<{inner_type}>"),
    }
}

fn rust_type(
    options: &Vec<ModelValueOption>,
    scope: ModelElementScope,
    model: &Model,
    _preset: bool,
) -> String {
    let inner_type = match get_values_type(options).expect("get params types") {
        ModelValueType::Single(s) => simple_to_rust_type(s).to_owned(),
        ModelValueType::Either(a, b) => {
            if a.is_bool() {
                format!("ToggleOr<{}>", simple_to_rust_type(b))
            } else if b.is_bool() {
                format!("ToggleOr<{}>", simple_to_rust_type(a))
            } else {
                format!(
                    "Either<{}, {}>",
                    simple_to_rust_type(a),
                    simple_to_rust_type(b)
                )
            }
        }
        ModelValueType::Any => "serde_json::Value".to_owned(),
    };

    let container_type = match scope {
        ModelElementScope::Global => "Vec",
        ModelElementScope::AllInputs => match model.inputs.len() {
            0 => return "()".to_owned(),
            1 => return inner_type,
            2 => "Stereo",
            _ => "Vec",
        },
        ModelElementScope::AllOutputs => match model.outputs.len() {
            0 => return "()".to_owned(),
            1 => return inner_type,
            2 => "Stereo",
            _ => "Vec",
        },
        ModelElementScope::Count(_) => "Vec",
    };

    format!("{container_type}<{inner_type}>")
}

fn pascal_case_converter() -> convert_case::Converter {
    convert_case::Converter::new().to_case(convert_case::Case::Pascal)
}

fn screaming_snake_case_converter() -> convert_case::Converter {
    convert_case::Converter::new().to_case(convert_case::Case::ScreamingSnake)
}

fn get_key<A, B>(a: &(A, B)) -> A
where
    A: Copy,
{
    a.0
}
