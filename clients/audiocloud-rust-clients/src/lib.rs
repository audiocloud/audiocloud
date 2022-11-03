use reqwest::{Client, ClientBuilder, Error};


pub use audio_engine::AudioEngineClient;
pub use domain_server::DomainServerClient;
pub use instance_driver::InstanceDriverClient;

pub(crate) fn create_client() -> Result<Client, Error> {
    ClientBuilder::default().brotli(true)
                            .use_rustls_tls()
                            .http2_keep_alive_while_idle(true)
                            .http2_keep_alive_interval(std::time::Duration::from_secs(1))
                            .http2_keep_alive_timeout(std::time::Duration::from_secs(5))
                            .tcp_keepalive(std::time::Duration::from_secs(60))
                            .tcp_nodelay(true)
                            .timeout(std::time::Duration::from_secs(20))
                            .build()
}

mod audio_engine;
mod domain_server;
mod instance_driver;

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize};

    #[tokio::test]
    async fn test_http_2() {
        let client = create_client().expect("Failed to create client");

        #[derive(Deserialize, Debug)]
        struct Http2StatusResponse {
            http2:      i32,
            protocol:   String,
            push:       i32,
            user_agent: String,
        }

        let res: Http2StatusResponse = client.get("https://http2.pro/api/v1")
                                             .send()
                                             .await
                                             .expect("Sent")
                                             .json()
                                             .await
                                             .expect("JSON");

        assert_eq!(res.http2, 1);
        assert_eq!(res.protocol.as_str(), "HTTP/2.0");

        drop(client);
    }
}
