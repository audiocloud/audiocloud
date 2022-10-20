export interface {{ ts_name }}Preset {
{%- for (property_id, property_spec) in model.parameters.iter() %}
    {{property_id}}: {{ (property_spec, model)|ts_param_type }},
{%- endfor -%}
}
