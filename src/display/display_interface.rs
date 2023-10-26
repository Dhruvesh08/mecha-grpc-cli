use anyhow::Error;
use tonic::transport::Channel;

#[allow(non_snake_case)]
pub mod displaymanager {
    tonic::include_proto!("displaymanager");
}

pub use displaymanager::{
    display_ctrl_service_client::DisplayCtrlServiceClient, GetBrightnessRequest,
    SetBrightnessRequest,
};

pub struct DisplayManagerClient {
    client: DisplayCtrlServiceClient<Channel>,
}

impl DisplayManagerClient {
    pub async fn new(url: String) -> Result<Self, Error> {
        let client = DisplayCtrlServiceClient::connect(url).await?;

        Ok(Self { client })
    }

    pub async fn get_brightness(&mut self) -> Result<(), Error> {
        let request = tonic::Request::new(GetBrightnessRequest {});
        let response = self.client.get_brightness(request).await?;

        println!("Brightness: {:?}", response.into_inner());

        Ok(())
    }

    pub async fn set_brightness(&mut self, brightness: u8) -> Result<(), Error> {
        let request = tonic::Request::new(SetBrightnessRequest {
            brightness: brightness as u32,
        });
        let response = self.client.set_brightness(request).await?;

        println!("Brightness: {:?}", response.into_inner());

        Ok(())
    }
}
