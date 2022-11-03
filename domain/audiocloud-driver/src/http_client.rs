/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use anyhow::anyhow;
use once_cell::sync::OnceCell;
use reqwest::Client;

static HTTP_CLIENT: OnceCell<Client> = OnceCell::new();

pub fn init() -> anyhow::Result<()> {
    HTTP_CLIENT.set(Client::new())
               .map_err(|_| anyhow!("HTTP client init already called!"))?;

    Ok(())
}

pub fn get_http_client() -> &'static Client {
    HTTP_CLIENT.get().expect("HTTP client not initialized!")
}
