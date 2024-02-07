use tonic::{transport::Server};
mod db_services;


use db_services::user_service::MyUserService;










#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    println!("Server listening on {}", addr);

    Server::builder()
        .serve(addr)
        .await?;

    Ok(())
}




