use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use std::net::SocketAddr;


pub enum PortStatus {
    Open,
    Closed,
    Timeout,
    InvalidAddress,
}

pub async fn port_scanner(ip: String, port: u16) -> PortStatus 

{
    let address = format!("{}:{}", ip, port);

    // check for valid input
    if address.parse::<SocketAddr>().is_err()
    {
        return PortStatus::InvalidAddress;
    }

    match timeout(Duration::from_millis(500), TcpStream::connect(&address)).await 
    {
        Ok(Ok(_)) => PortStatus::Open,
        Ok(Err(_)) => PortStatus::Closed,
        Err(_) => PortStatus::Timeout,
    }

}

// pub async fn full_port_scan







