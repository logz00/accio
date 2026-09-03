use tokio::task::JoinHandle;
use crate::port_scanner::{port_scanner, PortStatus};

pub async fn full_port_scan(ip: String) -> Vec<u16> 
{
    let mut handles: Vec<JoinHandle<(u16, PortStatus)>> = Vec::new();

    for port in 1..=65535 
    {
        let ip_clone = ip.clone();
        
        let handle = tokio::spawn(async move
        {
            let status = port_scanner(ip_clone, port).await;
            (port, status)

        });
    handles.push(handle);
    }

    let mut open_ports = Vec::new();

    for handle in handles 
    {
        if let Ok((port, status)) = handle.await
        {
            if let PortStatus::Open = status
            {
                open_ports.push(port);
            }
        }
    }
    open_ports
}
