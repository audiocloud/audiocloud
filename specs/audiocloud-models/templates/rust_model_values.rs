/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

[{%- for value in values -%}
{%- match value -%}
{%- when ModelValueOption::Single with (value) -%}
ModelValueOption::Single({{ ModelValueTemplate::new(value) }})
{%- when ModelValueOption::Range with (from, to) -%}
ModelValueOption::Range({{ ModelValueTemplate::new(from) }}, {{ ModelValueTemplate::new(to) }})
{%- endmatch -%},
{%- endfor -%}]