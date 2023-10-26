use anyhow::Error;
use clap::{Args, Subcommand};

use crate::cpu::cpu_interface::CpuManagerClient;

//create cpu args
#[derive(Debug, Args)]
pub struct Cpu {
    #[command(subcommand)]
    command: CpuCommands,
}

//create cpu subcommands
#[derive(Debug, Subcommand)]
enum CpuCommands {
    #[command(about = "Get cpu usage")]
    Usage,
    #[command(about = "Get cpu info")]
    Info,
}

impl Cpu {
    pub async fn execute(&self, uri: &str) -> Result<(), Error> {
        match &self.command {
            CpuCommands::Info => {
                //cpu info
                println!("Getting cpu info...");
                let mut client = CpuManagerClient::new(uri.to_string()).await?;
                client.get_cpu_info().await?;
            }
            CpuCommands::Usage => {
                //cpu usage
                println!("Getting cpu usage...");
                let mut client = CpuManagerClient::new(uri.to_string()).await?;
                client.get_cpu_usage().await?;
            }
        }
        Ok(())
    }
}
