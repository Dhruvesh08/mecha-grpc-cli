use clap::{Args, Parser, Subcommand};

mod network;

pub use network::NetworkManagerClient;

#[derive(Debug, Parser)]
#[command(name = "mecha")]
#[command(about = "A fictional Mecha CLI", long_about = None)]
struct MechaCli {
    #[command(subcommand)]
    command: Mecha,
}

#[derive(Debug, Subcommand)]
enum Mecha {
    #[command(about = "Interact with network interfaces")]
    Network(NetworkArgs),
}

#[derive(Debug, Args)]
struct NetworkArgs {
    #[command(subcommand)]
    command: Network,
}

#[derive(Debug, Subcommand)]
enum Network {
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

#[tokio::main]
async fn main() {
    let args = MechaCli::parse();

    match args.command {
        Mecha::Network(network_command) => match network_command.command {
            Network::Scan => {
                println!("Scanning for wireless networks...");
                //use this url  0.0.0.0:50052 for now

                let url = "http://0.0.0.0:50052".to_string(); // replace with your server's URL
                let mut network_manager_client = NetworkManagerClient::new(url).await.unwrap();
                network_manager_client
                    .scan_wireless_network()
                    .await
                    .unwrap();
                // Add your scan logic here or log the action.
            }
            Network::Add(args) => {
                println!(
                    "Adding wireless network: SSID={}, Password={}",
                    args.ssid, args.password
                );
                // Add your add network logic here or log the action.
            }
            Network::Remove(args) => {
                println!("Removing wireless network: SSID={}", args.ssid);
                // Add your remove network logic here or log the action.
            }
            Network::Connect(args) => {
                println!(
                    "Connecting to wireless network: SSID={}, Password={}",
                    args.ssid, args.password
                );
                // Add your connect network logic here or log the action.
            }
        },
    }

    // Continued program logic goes here...
}
