//use std::io::{self, Write};
use colorful::{Color, Colorful};
use clap::{Parser, Subcommand};
mod port_scanner;
use port_scanner::port_scanner;
use port_scanner::PortStatus;



fn main_logo() 
{
    let logo = r#"    _            _         _             _                    
   / \   ___ ___(_) ___   | |__  _   _  | |    ___   __ _ ____
  / _ \ / __/ __| |/ _ \  | '_ \| | | | | |   / _ \ / _` |_  /
 / ___ \ (_| (__| | (_) | | |_) | |_| | | |__| (_) | (_| |/ / 
/_/   \_\___\___|_|\___/  |_.__/ \__, | |_____\___/ \__, /___|
                                 |___/              |___/     
Welcome to the Accio Recon Tool || Creator: github.com/logz00"#;

    println!("{}", logo.gradient(Color::SeaGreen3).bold());
}


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
    }
}


// the magical loop
//
// for testing use -> cargo run -- scan (port) (ip)
fn main () 
{
    main_logo();
    
    let cli = Cli::parse();

    match cli.command {
    
        Commands::Scan {p, ip} => 
        {
            println!("Scanning target {ip} on port {p}...");
            match port_scanner(&ip, p) {
                PortStatus::Open => println!("Port {p} is OPEN"),
                PortStatus::Timeout => println!("Port {p} timed out"),
                PortStatus::Closed => println!("Port {p} is CLOSED"),
            }
        }

    }
}
