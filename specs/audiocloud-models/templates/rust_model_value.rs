/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

{%- match value %}
{%- when ModelValue::String with (value) %}
{%- when ModelValue::Number with (value) %}
ModelValue::Number({{value}}_f64)
{%- when ModelValue::Bool with (value) %}
ModelValue::Bool({{value}})
{%- endmatch -%}
