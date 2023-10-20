#![deny(clippy::all)]


use clap::{Args, Subcommand};
use std::error::Error;

// use crate::network::network_interface::NetworkManagerClient;

use super::network_interface::NetworkManagerClient;

#[derive(Debug, Args)]
#[command(name = "network")]
pub struct Network {
    #[command(subcommand)]
    pub command: NetworkCommand,
}

#[derive(Debug, Subcommand)]
pub enum NetworkCommand {
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
pub struct WirelessAddArgs {
    #[arg(required = true)]
    pub ssid: String,

    #[arg(required = true)]
    pub password: String,
}

#[derive(Debug, Args)]
pub struct WirelessRemoveArgs {
    #[arg(required = true)]
    pub ssid: String,
}

#[derive(Debug, Args)]
pub struct WirelessConnectArgs {
    #[arg(required = true)]
    pub ssid: String,

    #[arg(required = true)]
    pub password: String,
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
            NetworkCommand::Connect(_args) => {
                // Add your connect network logic here
            }
        }

        Ok(())
    }
}
