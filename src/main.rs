//add clippy
#![warn(clippy::all)]
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
    #[command(about = "Device memory info")]
    Memory(MemoryArgs),
    #[command(about = "Device storage info")]
    Storage(StorageArgs),
    #[command(about = "Device Cpu info")]
    Cpu(CpuArgs),
}


#[derive(Debug, Args)]
#[command(name = "network")]
struct NetworkArgs {
    #[command(subcommand)]
    command: Network,
}

#[derive(Debug, Args)]
struct MemoryArgs {
    #[command(subcommand)]
    command: Memory,
}

#[derive(Debug, Args)]
struct StorageArgs {
    #[command(subcommand)]
    command: Storage,
}

//create cpu args
#[derive(Debug, Args)]
struct CpuArgs {
    #[command(subcommand)]
    command: Cpu,
}

//create cpu subcommands
#[derive(Debug, Subcommand)]
enum Cpu {
    #[command(about = "Get cpu usage")]
    Usage,
    #[command(about = "Get cpu info")]
    Info,
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

#[derive(Debug, Subcommand)]
enum Memory {
    #[command(about = "Get memory usage")]
    Usage,
    #[command(about = "Get memory info")]
    Info,
}

#[derive(Debug, Subcommand)]
enum Storage {
    #[command(about = "Get storage usage")]
    Usage,
    #[command(about = "Get storage info")]
    Info,
}

#[tokio::main]
async fn main() {
    let args = MechaCli::parse();
    let url = "http://0.0.0.0:50052".to_string();

    match args.command {
        Mecha::Network(network_command) => match network_command.command {
            Network::Scan => {
                println!("Scanning for wireless networks...");
                //use this url  0.0.0.0:50052 for now
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
        Mecha::Memory(memory_command) => match memory_command.command {
            Memory::Usage => {
                println!("Getting memory usage...");
                // Add your scan logic here or log the action.
            }
            Memory::Info => {
                println!("Getting memory info...");
                // Add your add network logic here or log the action.
            }
        },
        Mecha::Storage(storage_command) => match storage_command.command {
            Storage::Usage => {
                println!("Getting storage usage...");
                // Add your scan logic here or log the action.
            }
            Storage::Info => {
                println!("Getting storage info...");
                // Add your add network logic here or log the action.
            }
        },
        Mecha::Cpu(cpu_command) => match cpu_command.command {
            Cpu::Usage => {
                println!("Getting cpu usage...");
                // Add your scan logic here or log the action.
            }
            Cpu::Info => {
                println!("Getting cpu info...");
                // Add your add network logic here or log the action.
            }
        },
    }

    // Continued program logic goes here...
}
