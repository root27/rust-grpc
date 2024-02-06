use tonic::{transport::Server, Request, Response, Status};
use std::env;


pub mod Helloworld {
    tonic::include_proto!("Helloworld");
}




use Helloworld::helloworld_server::{Helloworld, HelloworldServer};

use Helloworld::{HelloRequest, HelloReply};


#[derive(Default)]

pub struct MyHelloworld {}

#[tonic::async_trait]

impl Helloworld for MyHelloworld {
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = Helloworld::HelloReply {
            message: format!("Hello {}!", request.get_ref().name).into(),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:51000";

    let myhelloworld = MyHelloworld::default();

    Server::builder()
        .add_service(HelloworldServer::new(myhelloworld))
        .serve(addr.parse()?)
        .await?;

    Ok(())

}



