use lazy_static::lazy_static;
use reqwest::Client;

pub use super::Result;

lazy_static! {
  pub(crate) static ref HTTP_CLIENT: Client = Client::new();
}

mod download;
mod probe;
pub mod service;
mod upload;
