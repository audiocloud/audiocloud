/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

export interface {{ ts_name }}Preset {
{%- for (property_id, property_spec) in model.parameters.iter() %}
    {{property_id}}: {{ (property_spec, model)|ts_param_type }},
{%- endfor -%}
}
