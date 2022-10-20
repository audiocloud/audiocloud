use std::fs;

use serde_json::json;

use audiocloud_api::api::*;

fn main() {
    use utoipa::OpenApi;

    #[derive(OpenApi)]
    #[openapi()]
    struct ModelApi;

    // we generate a schema.json to be included in various builds

    fs::write(
        "openapi_models.json",
        openapi_with_schemas_to_json(
            ModelApi::openapi(),
            audiocloud_models::schemas(),
            json!([
                openapi_set_version("3.1.0"),
                openapi_add_apache_license(),
                openapi_set_info_title("Audio Cloud Models"),
            ]),
        )
        .expect("API convert to JSON"),
    )
    .expect("Write JSON to file");
}
