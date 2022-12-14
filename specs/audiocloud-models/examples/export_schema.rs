/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::fs::File;

fn main() -> anyhow::Result<()> {
    // we generate a Json Schema file to be used for validation, API generation, documentation etc.
    serde_yaml::to_writer(File::create("specs/openapi/models.yaml")?, &audiocloud_models::schemas())?;

    Ok(())
}
