use tonic::{transport::Server, Request, Response, Status};


pub mod helloworld {
    tonic::include_proto!("Helloworld");
}


#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]

impl helloworld::greeter_server::Greeter for MyGreeter {
    async fn say_hello(&self, request: Request<helloworld::HelloRequest>) -> Result<Response<helloworld::HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = helloworld::HelloReply {
            message: format!("Hello {}!", request.get_ref().name).into(),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(helloworld::greeter_server::GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}




