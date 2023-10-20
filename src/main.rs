//add clippy
#![warn(clippy::all)]
use clap::{Args, Parser, Subcommand};

mod network;

pub use network::Network;

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
    Network(Network),
    // #[command(about = "Device memory info")]
    // Memory(MemoryArgs),
    // #[command(about = "Device storage info")]
    // Storage(StorageArgs),
    // #[command(about = "Device Cpu info")]
    // Cpu(CpuArgs),
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
    // let url = "http://0.0.0.0:50052".to_string();

    match args.command {
        // we need to use execute command form network
        Mecha::Network(network) => {
            // let mut client = NetworkManagerClient::new(url).await.unwrap();
            network.execute().await.unwrap();
        }
    }

    // Continued program logic goes here...
}
