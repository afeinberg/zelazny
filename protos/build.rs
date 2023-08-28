fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = prost_build::Config::default();
    config.bytes(["."]);
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile_with_config(config, &["proto/tablet.proto"], &["proto/"])?;
    Ok(())
}
