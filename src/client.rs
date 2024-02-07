use serde_json::json;
use actix_web::{web,App, HttpServer, Responder, HttpResponse};


mod structs;

use structs::data::Data;


pub mod helloworld {
    tonic::include_proto!("helloworld");
}





async fn hello (data: web::Json<Data>) -> impl Responder {

    let mut client = helloworld::greeter_client::GreeterClient::connect("http://[::1]:50051").await.unwrap();



    let name = data.name.clone();


    let request = tonic::Request::new(helloworld::HelloRequest {
        name: name
    });

    let response = client.say_hello(request).await.unwrap();

    HttpResponse::Ok().json(json!(
        {
            "message": response.into_inner().message
        }
    
    ))

  
  
}


#[actix_web::main]


async fn main () -> std::io::Result<()> {



    println!("Server listening on {}", 8080);

    HttpServer::new(move || {
        App::new()
            .route("/", web::post().to(hello))
    })
    .bind(("127.0.0.1", 8080))
    ?
    .run()
    .await
}