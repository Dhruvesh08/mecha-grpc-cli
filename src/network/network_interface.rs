use tonic::transport::Channel;


#[allow(non_snake_case)]
pub mod networkmanager {
    tonic::include_proto!("networkmanager");
}

pub use networkmanager::{network_manager_service_client::NetworkManagerServiceClient,WifiConnectRequest,RemoveNetworkRequest,Empty};


pub struct NetworkManagerClient {
    client: NetworkManagerServiceClient<Channel>,
}


impl NetworkManagerClient {
    pub async fn new(url: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = NetworkManagerServiceClient::connect(url).await?;

        Ok(Self { client })
    }

    pub async fn scan_wireless_network(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let request = tonic::Request::new(Empty {});
        let response = self.client.scan_wireless_network(request).await?;
    
        println!("Scan Results: {:?}", response.into_inner());
    
        Ok(())
    }

    pub async fn connect_wireless_network(
        &mut self,
        ssid: &str,
        psk: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request = tonic::Request::new(WifiConnectRequest {
            ssid: ssid.to_string(),
            psk: psk.to_string(),
        });
        let response = self.client.connect_wireless_network(request).await?;

        println!("Connect Response: {:?}", response.into_inner());

        Ok(())
    }

    pub async fn disconnect_wireless_network(
        &mut self,
        network_id: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request = tonic::Request::new(RemoveNetworkRequest { network_id });
        let response = self.client.disconnect_wireless_network(request).await?;

        println!("Disconnect Response: {:?}", response.into_inner());

        Ok(())
    }
}

