use anyhow::Error;
use tonic::transport::Channel;

#[allow(non_snake_case)]
pub mod deviceinfo {
    tonic::include_proto!("deviceinfo");
}

#[allow(non_snake_case)]
pub mod metrics {
    tonic::include_proto!("metrics");
}

pub use deviceinfo::{device_info_service_client::DeviceInfoServiceClient, DiskInfo, Empty};

pub use metrics::{
    metrics_service_client::MetricsServiceClient, Empty as Matrics_Empty, GetDiskUsageResponse,
};

pub struct StorageManager {
    //add matrics client and device info client as we need both
    matrics_client: MetricsServiceClient<Channel>,
    device_info_client: DeviceInfoServiceClient<Channel>,
}

impl StorageManager {
    pub async fn new(url: String) -> Result<Self, Error> {
        let matrics_client = MetricsServiceClient::connect(url.clone()).await?;
        let device_info_client = DeviceInfoServiceClient::connect(url).await?;

        Ok(Self {
            matrics_client,
            device_info_client,
        })
    }

    pub async fn get_disk_usage(&mut self) -> Result<(), Error> {
        let request = tonic::Request::new(Matrics_Empty {});
        let response = self.matrics_client.get_disk_usage(request).await?;

        println!("Disk Usage: {:?}", response.into_inner());

        Ok(())
    }

    pub async fn get_disk_info(&mut self) -> Result<(), Error> {
        let request = tonic::Request::new(Empty {});
        let response = self.device_info_client.get_disk_info(request).await?;

        println!("Disk Info: {:?}", response.into_inner());

        Ok(())
    }
}
