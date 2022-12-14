#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct {{ rust_name }}Preset {
{%- for (property_id, property_spec) in model.parameters.iter().sorted_by_key(self::get_key) %}
    pub {{property_id}}: {{ (property_spec, model)|rust_preset_type }},
{%- endfor -%}
}
