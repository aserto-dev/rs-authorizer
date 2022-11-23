extern crate aserto_authorizer;

use aserto_authorizer::aserto::authorizer::v2::{authorizer_client::AuthorizerClient, InfoRequest};
use tonic::transport::{Certificate, Channel, ClientTlsConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load root cert (NOTE: location is MacOS specific)
    let pem = tokio::fs::read("/etc/ssl/cert.pem").await?;
    let ca = Certificate::from_pem(pem);

    let tls = ClientTlsConfig::new()
        .ca_certificate(ca)
        .domain_name("authorizer.prod.aserto.com");

    let channel = Channel::builder("http://authorizer.prod.aserto.com:8443".parse().unwrap())
        .tls_config(tls)?
        .connect()
        .await?;

    let mut client = AuthorizerClient::new(channel);

    let request = tonic::Request::new(InfoRequest {});

    let response = client.info(request).await?;

    dbg!(response);

    Ok(())
}
