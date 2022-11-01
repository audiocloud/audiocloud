#![allow(unused_imports)]

use audiocloud_api::model::*;
use audiocloud_api::api::*;
use serde::{Serialize, Deserialize};
use schemars::{JsonSchema, schema_for};
use schemars::schema::RootSchema;

{% for (manufacturer, this_models) in models.iter().sorted_by_key(self::get_key) %}
pub mod {{ manufacturer|lowercase }} {
use super::*;

pub const NAME: &str = "{{ manufacturer }}";

{% for (name, model) in this_models.iter().sorted_by_key(self::get_key) %}
{{ RustPresetModelTemplate::new(name, model) }}
{{ RustParamsModelTemplate::new(name, model) }}
{{ RustReportsModelTemplate::new(name, model) }}
pub mod {{name|lowercase}} {
use super::*;
pub const NAME: &str = "{{name}}";
{{ RustConstantsTemplate::new(model) }}
}
{% endfor %}
}
{% endfor %}

pub fn schemas() -> RootSchema {
    merge_schemas([
{%- for (manufacturer, this_models) in models.iter().sorted_by_key(self::get_key) %}
{%- for (name, model) in this_models.iter().sorted_by_key(self::get_key) %}
      schema_for!(self::{{manufacturer|lowercase}}::{{name|pascal_case}}Preset),
      schema_for!(self::{{manufacturer|lowercase}}::{{name|pascal_case}}Parameters),
      schema_for!(self::{{manufacturer|lowercase}}::{{name|pascal_case}}Reports),
{%- endfor %}
{%- endfor %}
    ].into_iter())
}
