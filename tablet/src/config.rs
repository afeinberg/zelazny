pub struct Config {
    /// The port to use for tablet server RPC.
    pub grpc_port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self { grpc_port: 8182 }
    }
}
