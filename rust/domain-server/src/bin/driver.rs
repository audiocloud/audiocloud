#[tokio::main]
async fn main() {
  domain_server::instance::service::DriverServer::new();
}
