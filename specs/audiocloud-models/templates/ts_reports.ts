/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

export interface {{ ts_name }}Reports {
{%- for (report_id, report_spec) in model.reports.iter() %}
    {{report_id}}: {{ (report_spec, model)|ts_report_type }},
{%- endfor -%}
}
