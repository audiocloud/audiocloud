#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
pub struct {{ rust_name }}Parameters {
{%- for (property_id, property_spec) in model.parameters.iter().sorted_by_key(self::get_key) %}
    pub {{property_id}}: Option<{{ (property_spec, model)|rust_param_type }}>,
{%- endfor -%}
}
