use std::net::{TcpStream, SocketAddr};
use std::time::Duration;
use std::io::ErrorKind;



pub enum PortStatus {
    Open,
    Closed,
    Timeout,
}

pub fn port_scanner(ip: &str, port: u16) -> PortStatus 
{
    let address = format!("{}:{}", ip, port);
    
    let socket_addr: SocketAddr = match address.parse() {
        Ok(addr) => addr,
        Err(_) => return PortStatus::Closed,
    };
    
    match TcpStream::connect_timeout(&socket_addr, Duration::from_millis(500)){
        Ok(_) => PortStatus::Open,
        Err(e) => {
            if e.kind() == ErrorKind::TimedOut {
                PortStatus::Timeout
            } else {
                PortStatus::Closed
            }
        },
    }

}


