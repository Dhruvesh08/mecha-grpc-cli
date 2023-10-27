//add clippy
#![warn(clippy::all)]
use std::{fs::File, io::BufReader};

use clap::{Parser, Subcommand};

mod network;
pub use network::Network;

mod memory;
pub use memory::Memory;

mod storage;
pub use storage::Storage;

mod cpu;
pub use cpu::Cpu;

mod display;
pub use display::Display;

mod configs;
pub use configs::BaseConfig;

#[derive(Debug, Parser)]
#[command(name = "mecha")]
#[command(about = "A fictional Mecha CLI", long_about = None)]
struct MechaCli {
    #[command(subcommand)]
    command: Mecha,
}

#[derive(Debug, Subcommand)]
enum Mecha {
    #[command(about = "Interact with network utility")]
    Network(Network),
    #[command(about = "Device memory utility")]
    Memory(Memory),
    #[command(about = "Device storage utility")]
    Storage(Storage),
    #[command(about = "Device Cpu utility")]
    Cpu(Cpu),
    #[command(about = "Device display utility")]
    Display(Display),
}

#[tokio::main]
async fn main() {
    let profile_file = File::open("./Config.yaml").expect("Failed to open config file");
    let reader = BufReader::new(profile_file);

    let config: BaseConfig = serde_yaml::from_reader(reader).expect("unable to rad yaml file");

    //port for grpc server
    let port = config.server.port;
    let uri = config.server.url;

    let url = format!("http://{}:{}", uri, port);

    let args = MechaCli::parse();

    match args.command {
        // we need to use execute command form network
        Mecha::Network(network) => {
            // let mut client = NetworkManagerClient::new(url).await.unwrap();
            network.execute().await.unwrap();
        }
        Mecha::Memory(memory) => {
            memory.execute().await.unwrap();
        }
        Mecha::Storage(storage) => {
            storage.execute(&url).await.unwrap();
        }
        Mecha::Cpu(cpu) => {
            cpu.execute(&url).await.unwrap();
        }
        Mecha::Display(display) => {
            display.execute(&url).await.unwrap();
        }
    }

    // Continued program logic goes here...
}
