use anyhow::Error;
use clap::{Args, Subcommand};

use crate::display::display_interface::DisplayManagerClient;

#[derive(Debug, Args)]
pub struct Display {
    #[command(subcommand)]
    command: DisplayCommands,
}

#[derive(Debug, Subcommand)]
enum DisplayCommands {
    #[command(about = "Get display brightness")]
    GetBrightness,
    #[command(about = "Set display brightness")]
    SetBrightness(SetBrightnessArgs),
}

#[derive(Debug, Args)]
struct SetBrightnessArgs {
    #[arg(required = true)]
    brightness: u8,
}

impl Display {
    pub async fn execute(&self, uri: &str) -> Result<(), Error> {
        match &self.command {
            DisplayCommands::GetBrightness => {
                //get display brightness
                println!("Getting display brightness...");
                let mut client = DisplayManagerClient::new(uri.to_string()).await?;
                client.get_brightness().await?;
            }
            DisplayCommands::SetBrightness(args) => {
                //set display brightness
                println!("Setting display brightness...");
                let mut client = DisplayManagerClient::new(uri.to_string()).await?;
                client.set_brightness(args.brightness).await?;
            }
        }
        Ok(())
    }
}
