use clap::{Args, Subcommand};
use std::error::Error;

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
    pub async fn execute(&self) -> Result<(), Box<dyn Error>> {
        match &self.command {
            MemoryCommands::Info => {
                //memory info
                print!("Device Memory info")
            }
            MemoryCommands::Usage => {
                //memory usage
                print!("Devide memory usage")
            }
        }
        Ok(())
    }
}
