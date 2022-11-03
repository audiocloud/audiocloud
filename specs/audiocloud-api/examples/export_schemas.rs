/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::fs::{self, File};
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use schemars::schema::RootSchema;
use serde_yaml::Value;

use audiocloud_api::{audio_engine, cloud, domain, instance_driver};

fn update_schemas_of_openapi_file(api_file: PathBuf, schemas: RootSchema) -> Result<()> {
    let mut yaml: Value = serde_yaml::from_reader(File::open(&api_file)?)?;
    yaml.get_mut("components")
        .ok_or_else(|| anyhow!("no components in openapi file"))?
        .as_mapping_mut()
        .ok_or_else(|| anyhow!("components is not a mapping"))?
        .insert(Value::String("schemas".to_owned()), serde_yaml::to_value(schemas.definitions)?);

    let as_string = serde_yaml::to_string(&yaml)?.replace("#/definitions/", "#/components/schemas/");

    fs::write(api_file, as_string)?;

    Ok(())
}

fn export_cloud_openapi() -> Result<()> {
    update_schemas_of_openapi_file(PathBuf::from("specs/openapi/cloud_api.yaml"), cloud::schemas())
}

fn export_audio_engine_openapi() -> Result<()> {
    update_schemas_of_openapi_file(PathBuf::from("specs/openapi/audio_engine_api.yaml"), audio_engine::schemas())
}

fn export_instance_driver_openapi() -> Result<()> {
    update_schemas_of_openapi_file(PathBuf::from("specs/openapi/instance_driver_api.yaml"), instance_driver::schemas())
}

fn export_domain_openapi() -> Result<()> {
    update_schemas_of_openapi_file(PathBuf::from("specs/openapi/domain_api.yaml"), domain::schemas())
}

fn main() -> Result<()> {
    export_cloud_openapi()?;
    export_audio_engine_openapi()?;
    export_instance_driver_openapi()?;
    export_domain_openapi()?;

    Ok(())
}
