fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .compile(
            &[
                "proto/common.proto", 
                "proto/instruments.proto",
                "proto/marketdata.proto",
                "proto/operations.proto",
                "proto/orders.proto",
                "proto/sandbox.proto",
                "proto/stoporders.proto",
                "proto/users.proto"],
            &["proto"])?;

    Ok(())
}