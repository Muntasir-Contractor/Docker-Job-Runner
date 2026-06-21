use bollard::Docker;
use std::process::Command;
use futures_util::future::TryFutureExt;

#[tokio::main]
async fn main() {
    let conn = Docker::connect_with_socket_defaults().unwrap();
    match conn.ping().await{
        Ok(_) => println!("Connected"),
        Err(e) => println!("{}",e)
    };
}
