/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use serde_json::Value;

pub fn merge_values(maybe_object: &mut Value, other: Value) {
    if let (Value::Object(object), Value::Object(other)) = (maybe_object, other) {
        object.extend(other.into_iter());
    }
}

pub fn empty_object() -> Value {
    Value::Object(serde_json::Map::new())
}
