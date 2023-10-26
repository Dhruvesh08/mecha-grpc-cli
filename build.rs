fn main() -> Result<(), Box<dyn std::error::Error>> {
    let network_manager = "./proto/network_manager.proto";
    let display_manager = "./proto/display_manager.proto";
    let device_metrics = "./proto/metrics_manager.proto";
    let cpu_governor_ctrl = "./proto/cpu_governor_ctrl.proto";
    let device_info = "./proto/device_info.proto";

    tonic_build::configure()
        .build_server(true)
        .type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize,)]")
        .compile(
            &[
                network_manager,
                display_manager,
                device_metrics,
                cpu_governor_ctrl,
                device_info
            ],
            &["."],
        )
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    Ok(())
}
