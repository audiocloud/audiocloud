{%- match value %}
{%- when ModelValue::String with (value) %}
{%- when ModelValue::Number with (value) %}
ModelValue::Number({{value}}_f64)
{%- when ModelValue::Bool with (value) %}
ModelValue::Bool({{value}})
{%- endmatch -%}
