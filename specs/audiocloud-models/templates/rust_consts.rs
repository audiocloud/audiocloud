/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

{%- for (property_id, property_spec) in model.parameters.iter().sorted_by_key(self::get_key) %}
pub const {{property_id|screaming_snake}}_NAME: &str = "{{ property_id }}";
pub const {{property_id|screaming_snake}}_VALUES: [ModelValueOption; {{ property_spec.values.len() }}] = {{ ModelValueOptionsTemplate::new(property_spec.values) }};
{%- endfor -%}