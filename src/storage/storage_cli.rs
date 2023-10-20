use anyhow::Error;
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct Storage {
    #[command(subcommand)]
    command: StorageCommands,
}

#[derive(Debug, Subcommand)]
enum StorageCommands {
    #[command(about = "Get storage usage")]
    Usage,
    #[command(about = "Get storage info")]
    Info,
}

impl Storage {
    pub async fn execute(&self) -> Result<(), Error> {
        match self.command {
            StorageCommands::Info => {
                println!("Storage info")
            }
            StorageCommands::Usage => {
                println!("Storage usage")
            }
        }

        Ok(())
    }
}
