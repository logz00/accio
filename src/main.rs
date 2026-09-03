use clap::{Parser, Subcommand};
mod port_scanner;
use port_scanner::port_scanner;
use port_scanner::PortStatus;
mod logo;
use logo::main_logo;
mod full_portscan;
use full_portscan::full_port_scan;
use std::time::Instant;


// command line setup to be cool
#[derive(Parser)]
#[command(name = "accio")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Scan {
        p: u16,
        ip: String,
    },
    Fullscan {
        ip: String,
    }
}


// magical main loop using tokio
//
// for testing use -> cargo run -- scan (port) (ip)
#[tokio::main]
async fn main()
{
    main_logo();
    
    let cli = Cli::parse();

    match cli.command {
    
        Commands::Scan {p, ip} => 
        {
            println!("[*] Scanning {ip} on Port {p}...\n");

            match port_scanner(ip, p).await
            {
                PortStatus::Open => println!("Port {p} => OPEN [🟢]\n"),
                PortStatus::Timeout => println!("Port {p} => TIMEOUT [🟡]\n"),
                PortStatus::Closed => println!("Port {p} => CLOSED [🔴]\n"),
                PortStatus::InvalidAddress => println!("Invalid IP Address Format\n"),
            }
        }
        
        Commands::Fullscan {ip} => 
        {
            // track time
            let start = Instant::now();
            println!("\x1b[3m[*] Scanning {ip} for Open Ports...\x1b[0m\n");

            let open_ports = full_port_scan(ip).await;

            for port in open_ports
            {
                print!("Port {port} => OPEN [🟢]\n");
            }
            let elapsed = start.elapsed();

            let timer = format!("{:?}", elapsed);
            println!("\n\x1b[3m[!] Process finished in {:.4} seconds.\x1b[0m", timer);
        }
    }
}
