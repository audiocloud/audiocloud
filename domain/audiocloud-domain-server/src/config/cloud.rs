/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use reqwest::{Client, Url};
use tracing::*;

use audiocloud_api::cloud::domains::DomainConfig;

#[instrument(skip_all, err)]
pub async fn get_config(url: Url, api_key: String) -> anyhow::Result<DomainConfig> {
    let client = Client::new();
    let url = url.join("/v1/domains/config")?;

    Ok(client.get(url).bearer_auth(api_key).send().await?.json::<DomainConfig>().await?)
}
