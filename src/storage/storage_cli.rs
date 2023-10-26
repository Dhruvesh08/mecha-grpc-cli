use anyhow::Error;
use clap::{Args, Subcommand};

use crate::storage::storage_interface::StorageManager;

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
    pub async fn execute(&self, uri: &str) -> Result<(), Error> {
        match self.command {
            StorageCommands::Info => {
                println!("Storage info");
                let mut client = StorageManager::new(uri.to_string()).await?;
                client.get_disk_info().await?;
            }
            StorageCommands::Usage => {
                println!("Storage usage");
                let mut client = StorageManager::new(uri.to_string()).await?;
                client.get_disk_usage().await?;
            }
        }

        Ok(())
    }
}
