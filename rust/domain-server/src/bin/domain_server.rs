use domain_server::Result;

#[tokio::main]
async fn main() -> Result {
  println!("audiocloud domain server, version {}", env!("CARGO_PKG_VERSION"));

  Ok(())
}
