use tonic::{transport::Server};
mod db_services;


use db_services::user_service::MyUserService;
use db_services::user_service::user::user_service_server;










#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    let service = MyUserService::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(user_service_server::UserServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}




