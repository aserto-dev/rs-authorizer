extern crate aserto_authorizer;
extern crate tokio;
extern crate tonic;

use aserto_authorizer::aserto::authorizer::v2::{authorizer_client::AuthorizerClient, InfoRequest};
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Identity};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let certs_dir = dirs::home_dir().unwrap().join(".config").join("topaz").join("certs");

    let server_root_ca_cert = tokio::fs::read(certs_dir.join("grpc-ca.crt")).await?;
    let server_root_ca_cert = Certificate::from_pem(server_root_ca_cert);

    let client_cert = tokio::fs::read(certs_dir.join("grpc.crt")).await?;
    let client_key = tokio::fs::read(certs_dir.join("grpc.key")).await?;

    let client_identity = Identity::from_pem(client_cert, client_key);

    let tls = ClientTlsConfig::new()
        .domain_name("localhost")
        .ca_certificate(server_root_ca_cert)
        .identity(client_identity);

    let channel = Channel::builder("https://0.0.0.0:8282".parse().unwrap())
        .tls_config(tls)?
        .connect()
        .await?;

    let mut client = AuthorizerClient::new(channel);

    let request = tonic::Request::new(InfoRequest {});

    let response = client.info(request).await?;

    dbg!(response);

    Ok(())

}
