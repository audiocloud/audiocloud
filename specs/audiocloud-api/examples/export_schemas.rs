use serde_json::json;
use std::fs;
use utoipa::OpenApi;

use audiocloud_api::api::*;

fn export_cloud_openapi() {
    use audiocloud_api::cloud::*;

    fs::write(
        "openapi_cloud.json",
        openapi_with_schemas_to_json(
            CloudApi::openapi(),
            schemas(),
            json!([
                openapi_set_version("3.1.0"),
                openapi_add_apache_license(),
                openapi_set_info_title("Audio Cloud Orchestrator"),
                openapi_create_empty_servers(),
                openapi_add_server("https://api.audiocloud.io", "Production"),
                openapi_add_server("http://localhost:7100", "Local development"),
            ]),
        )
        .expect("API convert to JSON"),
    )
    .expect("Write JSON to file");
}

fn export_audio_engine_openapi() {
    use audiocloud_api::audio_engine::*;

    fs::write(
        "openapi_audio_engine.json",
        openapi_with_schemas_to_json(
            EngineApi::openapi(),
            schemas(),
            json!([
                openapi_set_version("3.1.0"),
                openapi_add_apache_license(),
                openapi_set_info_title("Audio Cloud Audio Engine"),
                openapi_create_empty_servers(),
                openapi_add_server("http://localhost:7300", "Local development")
            ]),
        )
        .expect("API convert to JSON"),
    )
    .expect("Write JSON to file");
}

fn export_instance_driver_openapi() {
    use audiocloud_api::instance_driver::*;

    fs::write(
        "openapi_instance_driver.json",
        openapi_with_schemas_to_json(
            InstanceDriverApi::openapi(),
            schemas(),
            json!([
                openapi_set_version("3.1.0"),
                openapi_add_apache_license(),
                openapi_set_info_title("Audio Cloud Instance Driver"),
                openapi_create_empty_servers(),
                openapi_add_server("http://localhost:7400", "Local development")
            ]),
        )
        .expect("API convert to JSON"),
    )
    .expect("Write JSON to file");
}

fn export_domain_openapi() {
    use audiocloud_api::domain::*;

    fs::write(
        "openapi_domain.json",
        openapi_with_schemas_to_json(
            DomainApi::openapi(),
            schemas(),
            json!([
                openapi_set_version("3.1.0"),
                openapi_add_apache_license(),
                openapi_set_info_title("Audio Cloud Domain"),
                openapi_create_empty_servers(),
                openapi_add_server("https://distopik-hq.eu.audiocloud.io", "Distopik HQ"),
                openapi_add_server("http://localhost:7200", "Local development")
            ]),
        )
        .expect("API convert to JSON"),
    )
    .expect("Write JSON to file");
}

fn main() {
    export_cloud_openapi();
    export_audio_engine_openapi();
    export_instance_driver_openapi();
    export_domain_openapi();
}
