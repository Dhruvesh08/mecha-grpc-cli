fn main() -> Result<(), Box<dyn std::error::Error>> {
    let network_manager = "./proto/network_manager.proto";
    // tonic_build::configure()
    //     .build_client(true)
    //     .compile(&[network_manager], &["./proto/network_manager.proto"])?;
    // Ok(())

    tonic_build::configure()
        .build_server(true)
        // .out_dir("src/")
        // adding attributes
        .type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize,)]")
        .compile(&[network_manager], &["."])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    Ok(())
}
