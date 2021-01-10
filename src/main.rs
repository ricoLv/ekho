mod config;
mod icmp;
mod kcp;
mod session;
mod socks5;

use crate::config::get_config;
use anyhow::Result;
use std::env;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::DEBUG).init();

    let config_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("config.toml"));

    info!("Ekho (exp. async. impl.) by Chengyuan Ma");

    config::load_config_from_file(config_path).await?;
    icmp::init_send_recv_loop().await?;
    session::init_recv_loop().await;

    if get_config().remote.is_none() {
        let mut session = session::Session::incoming().await;
        info!("received session: {}", session);
        session.sender.send(vec![1, 1, 4, 5, 1, 4]).await.unwrap();
        info!("data sent");
        session.close().await;
        info!("closed");
    // server::test_file_upload();
    } else {
        let mut session = session::Session::new(get_config().remote.unwrap(), 998244353);
        session.sender.send(vec![1]).await.unwrap();
        let res = session.receiver.recv().await.unwrap();
        info!("received: {:?}", res);
        session.close().await;
        info!("closed");
        // client::test_file_download();
    }
    Ok(())
}
