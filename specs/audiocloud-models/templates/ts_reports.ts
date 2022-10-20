export interface {{ ts_name }}Reports {
{%- for (report_id, report_spec) in model.reports.iter() %}
    {{report_id}}: {{ (report_spec, model)|ts_report_type }},
{%- endfor -%}
}
