use serde_json::json;
use tonic::Request;
use actix_web::{web,App, HttpServer, Responder, HttpResponse};

pub mod helloworld {
    tonic::include_proto!("helloworld");
}




async fn hello () -> impl Responder {

    let mut client = helloworld::greeter_client::GreeterClient::connect("http://[::1]:50051").await.unwrap();




    let request = Request::new(helloworld::HelloRequest {
        name: "Tonic".into(),
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
            .route("/", web::get().to(hello))
    })
    .bind(("127.0.0.1", 8080))
    ?
    .run()
    .await
}