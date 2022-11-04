/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix_web::middleware::Logger;
use actix_web::web::ServiceConfig;
use actix_web::{App, HttpServer};
use openssl::pkey::PKey;
use openssl::ssl::{SslAcceptor, SslMethod};
use openssl::x509::X509;

/// Starts the actix server with self signed HTTP2 server
///
/// # Arguments
///
/// * `bind`: The address to bind to
/// * `port`: The port to bind to
/// * `configure`: The function to configure the server
///
/// returns: Result<(), Error> - Ok when server exits cleanly, or an error if initialization fails
pub async fn start_http2_server<F>(bind: &str, port: u16, configure: F) -> anyhow::Result<()>
    where F: FnOnce(&mut ServiceConfig) + Clone + Send + Sync + 'static
{
    let certificate = rcgen::generate_simple_self_signed(vec!["localhost".to_owned(), bind.to_owned()])?;
    let mut ssl_config = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    let private_key = PKey::private_key_from_der(certificate.serialize_private_key_der().as_slice())?;
    let certificate = X509::from_der(&certificate.serialize_der()?)?;

    ssl_config.set_private_key(&private_key)?;
    ssl_config.set_certificate(&certificate)?;

    HttpServer::new(move || App::new().wrap(Logger::default()).configure(configure.clone())).bind_openssl((bind, port), ssl_config)?
                                                                                            .run()
                                                                                            .await?;

    Ok(())
}
