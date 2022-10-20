use std::collections::HashSet;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::common::ModelId;

#[derive(Serialize, Deserialize, Debug, Clone, Default, JsonSchema)]
pub struct ModelFilter {
    pub manufacturer_is: Option<String>,
    pub name_contains:   Option<String>,
    pub id_one_of:       HashSet<ModelId>,
}

impl ModelFilter {
    pub fn with_id_one_of(mut self, ids: impl IntoIterator<Item = ModelId>) -> Self {
        self.id_one_of.extend(ids);
        self
    }

    pub fn with_manufacturer_is(mut self, manufacturer: impl Into<String>) -> Self {
        self.manufacturer_is = Some(manufacturer.into());
        self
    }

    pub fn with_name_contains(mut self, name: impl Into<String>) -> Self {
        self.name_contains = Some(name.into());
        self
    }
}
