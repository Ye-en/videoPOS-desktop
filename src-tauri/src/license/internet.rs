use std::net::SocketAddr;
use std::time::Duration;
use anyhow::Result;
use tokio::net::TcpStream;
use tokio;
use log::error;

const INTERNET_CHECK_HOST: &str = "8.8.8.8";
const INTERNET_CHECK_PORT: &u16 = &53;

pub async fn online(timeout: &u64) -> bool {
    let socket_address: Result<SocketAddr, _> = format!("{}:{}", INTERNET_CHECK_HOST, INTERNET_CHECK_PORT).parse();

    let socket_address = match socket_address {
        Ok(addr) => addr,
        Err(e) => {
            error!("wrong socket address {e}");
            return false;
        }
    };

    match tokio::time::timeout(Duration::from_secs(timeout.to_owned()), TcpStream::connect(&socket_address)).await {
        Ok(result) => {
            match result {
                Ok(_) => {
                    return true;
                },
                _ => {
                    return false;
                }
            }
        }
        Err(_) => {
            return false;
        }
    };
}