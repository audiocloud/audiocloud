/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

import * as T from "./types"

{% for (manufacturer, this_models) in models.iter() %}
{% for (name, model) in this_models.iter() %}
{{ TSPresetModelTemplate::new(name, model) }}
{{ TSParamsModelTemplate::new(name, model) }}
{{ TSReportsModelTemplate::new(name, model) }}
{% endfor %}
{% endfor %}
