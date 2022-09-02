use authuser::{
    pb::{self},
    AuthenticationServer,
};
use tonic::{
    metadata::MetadataValue,
    transport::{Identity, Server, ServerTlsConfig},
    Request, Status,
};

#[macro_use]
extern crate lazy_static;

mod authuser;
mod constants;
mod db;
mod routes;

mod operations;
mod validations;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    // let t = &operations::DynamoDBClient;

    let cert = tokio::fs::read("./certs/pub.pem").await?;
    let key = tokio::fs::read("./certs/priv.key").await?;

    let identity = Identity::from_pem(cert, key);

    let addr = "[::1]:50051".parse().unwrap();

    // let server = EchoServer::default();
    let server2 = AuthenticationServer::default();

    // let svc = pb::echo_server::EchoServer::with_interceptor(server, check_auth);
    let svc2 = pb::authentication_server::AuthenticationServer::new(server2);

    let web_rpc = tonic_web::enable(svc2);

    Server::builder()
        .tls_config(ServerTlsConfig::new().identity(identity))?
        // .add_service(svc2)
        // .add_service(svc2)
        .add_service(web_rpc)
        .serve(addr)
        .await?;

    Ok(())
}

