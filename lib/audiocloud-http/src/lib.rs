use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;

use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use axum_server::{bind, bind_rustls};
use clap::Args;
use tower_http::trace::TraceLayer;

#[derive(Debug, Clone, Args)]
pub struct HttpOpts {
    /// HTTP port to listen instead of default
    #[clap(long, env)]
    pub http_port: Option<u16>,

    /// IP address to bind to and listen on
    #[clap(long, env, default_value = "0.0.0.0")]
    pub http_bind: IpAddr,

    /// Path to a certificate
    #[clap(long, env)]
    pub http_certificate: Option<PathBuf>,

    /// Path to a private key
    #[clap(long, env)]
    pub http_private_key: Option<PathBuf>,

    /// If set, the server will generate self-signed certificate and private key
    #[clap(long, env)]
    pub http_generate_certificates: bool,
}

pub async fn http_server<S: Clone + Send + Sync + 'static>(opts: &HttpOpts, default_port: u16, router: Router<S>) {
    let port = opts.http_port.unwrap_or(default_port);

    let addr = SocketAddr::from((opts.http_bind, port));

    let router = router.layer(TraceLayer::new_for_http());

    match (&opts.http_certificate, &opts.http_private_key, opts.http_generate_certificates) {
        (Some(certificate), Some(private_key), _) => {
            let cfg = RustlsConfig::from_pem_file(certificate, private_key).await
                                                                           .expect("TLS configuration");

            bind_rustls(addr, cfg).serve(router.into_make_service())
                                  .await
                                  .expect("HTTP server exit cleanly")
        }
        (_, _, true) => {
            let certificate =
                rcgen::generate_simple_self_signed(vec!["localhost".to_owned(), opts.http_bind.to_string()]).expect("Generate certificate");

            let cfg = RustlsConfig::from_pem(certificate.serialize_pem().expect("Generate certificate PEM").into_bytes(),
                                             certificate.serialize_private_key_pem().into_bytes()).await
                                                                                                  .expect("TLS configuration");

            bind_rustls(addr, cfg).serve(router.into_make_service())
                                  .await
                                  .expect("HTTP server exit cleanly")
        }
        _ => bind(addr).serve(router.into_make_service())
                       .await
                       .expect("HTTP server exit cleanly"),
    };
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use std::time::Duration;

    use axum::routing::get;
    use tokio::spawn;
    use tokio::time::timeout;

    use super::*;

    #[test_log::test(tokio::test)]
    async fn test_http1_server() {
        let opts = HttpOpts { http_port:                  None,
                              http_bind:                  IpAddr::from_str("0.0.0.0").expect("IP address"),
                              http_certificate:           None,
                              http_private_key:           None,
                              http_generate_certificates: false, };

        let router = Router::new().route("/", get(hello_world));
        let handle = spawn(async move { timeout(Duration::from_secs(3), http_server(&opts, 8101, router)).await });

        // TODO: make a request to the server and check the response

        handle.await.expect("Should time out gracefully");
    }

    #[test_log::test(tokio::test)]
    async fn test_http2_server() {
        let opts = HttpOpts { http_port:                  None,
                              http_bind:                  IpAddr::from_str("0.0.0.0").expect("IP address"),
                              http_certificate:           Some(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("self_signed_certs")
                                                                                                        .join("cert.pem")),
                              http_private_key:           Some(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("self_signed_certs")
                                                                                                        .join("key.pem")),
                              http_generate_certificates: false, };

        let router = Router::new().route("/", get(hello_world));
        let handle = spawn(async move { timeout(Duration::from_secs(3), http_server(&opts, 8102, router)).await });

        // TODO: make a request to the server and check the response

        handle.await.expect("Should time out gracefully");
    }

    #[test_log::test(tokio::test)]
    async fn test_http2_self_signed_server() {
        let opts = HttpOpts { http_port:                  None,
                              http_bind:                  IpAddr::from_str("0.0.0.0").expect("IP address"),
                              http_certificate:           None,
                              http_private_key:           None,
                              http_generate_certificates: true, };

        let router = Router::new().route("/", get(hello_world));
        let handle = spawn(async move { timeout(Duration::from_secs(3), http_server(&opts, 8103, router)).await });

        // TODO: make a request to the server and check the response

        handle.await.expect("Should time out gracefully");
    }

    async fn hello_world() -> &'static str {
        "Hello, World!"
    }
}
