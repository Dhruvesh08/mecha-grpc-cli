use anyhow::Error;
use clap::{Args, Subcommand};

use crate::memory::memory_interface::MemoryManagerClient;

#[derive(Debug, Args)]
pub struct Memory {
    #[command(subcommand)]
    command: MemoryCommands,
}

#[derive(Debug, Subcommand)]
enum MemoryCommands {
    #[command(about = "Get memory usage")]
    Usage,
    #[command(about = "Get memory info")]
    Info,
}

impl Memory {
    pub async fn execute(&self) -> Result<(), Error> {
        match &self.command {
            MemoryCommands::Info => {
                //memory info
                println!("Getting memory info...");
                let mut client =
                    MemoryManagerClient::new("http://localhost:50052".to_string()).await?;
                client.get_memory_info().await?;
            }
            MemoryCommands::Usage => {
                //memory usage
                println!("Getting memory usage...");
                let mut client =
                    MemoryManagerClient::new("http://localhost:50052".to_string()).await?;
                client.get_memory_usage().await?;
            }
        }
        Ok(())
    }
}
