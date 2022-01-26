# Пример
## `Cargo.toml`
```
[dependencies]
tokio = {version = "1.15.0", features = ["full"]}
tonic = {version = "0.6.2", features = ["transport", "tls-roots"]}
prost = "0.9.0"
prost-types = "0.9.0" # Contains definitions of Protocol Buffers well-known types
```
## `src/main.rs`
```
use std::str::FromStr;
use tonic::metadata::{MetadataKey, MetadataValue};
use tonic::transport::{ClientTlsConfig};
use tonic::{transport::Channel, Request};
use tinkoff_investments::{InstrumentsRequest, channel};
use tinkoff_investments::instruments_service_client::InstrumentsServiceClient;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = channel().await?;

    let token = MetadataValue::from_str("Bearer <token>")?;
    let key = MetadataKey::from_str("Authorization")?;

    let mut client = InstrumentsServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert(key.clone(), token.clone());
        Ok(req)
    });
    
    let request = tonic::Request::new(InstrumentsRequest{
        instrument_status: 1
    });

    let response = client.currencies(request).await?;

    println!("Response={:?}", response.get_ref());

    Ok(())
}
```