#![deny(clippy::all)]

use clap::{Args, Subcommand};
use std::error::Error;

use crate::network::network_interface::NetworkManagerClient;

#[derive(Debug, Args)]
#[command(name = "network")]
pub struct Network {
    #[command(subcommand)]
    command: NetworkCommand,
}

#[derive(Debug, Subcommand)]
enum NetworkCommand {
    #[command(about = "Scan for wireless networks")]
    Scan,

    #[command(about = "Add a wireless network")]
    Add(WirelessAddArgs),

    #[command(about = "Remove a wireless network")]
    Remove(WirelessRemoveArgs),

    #[command(about = "Connect to a wireless network")]
    Connect(WirelessConnectArgs),
}

#[derive(Debug, Args)]
struct WirelessAddArgs {
    #[arg(required = true)]
    ssid: String,

    #[arg(required = true)]
    password: String,
}

#[derive(Debug, Args)]
struct WirelessRemoveArgs {
    #[arg(required = true)]
    ssid: String,
}

#[derive(Debug, Args)]
struct WirelessConnectArgs {
    #[arg(required = true)]
    ssid: String,

    #[arg(required = true)]
    password: String,
}

impl Network {
    pub async fn execute(&self) -> Result<(), Box<dyn Error>> {
        match &self.command {
            NetworkCommand::Scan => {
                // Add your scan logic here
                //moke 5 wireless networks
                println!("Scanning for wireless networks...");
                let networks = vec![
                    "Network 1".to_string(),
                    "Network 2".to_string(),
                    "Network 3".to_string(),
                    "Network 4".to_string(),
                    "Network 5".to_string(),
                ];

                //use network interface to scan
                let uri = "http://www.google.com".to_string();
                let mut client = NetworkManagerClient::new(uri).await?;
                client.scan_wireless_network().await?;
                // println!("Scan Results: {:?}", response.into_inner());

                println!("Scan Results: {:?}", networks);
            }
            NetworkCommand::Add(_args) => {
                // Add your add network logic here
            }
            NetworkCommand::Remove(_args) => {
                // Add your remove network logic here
            }
            NetworkCommand::Connect(args) => {
                // Add your connect network logic here
                println!("Connecting to wireless network: {}", args.ssid);
            }
        }

        Ok(())
    }
}
