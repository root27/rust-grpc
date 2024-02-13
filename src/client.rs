use std::clone;

use serde_json::json;
use actix_web::{web,App, HttpServer, Responder, HttpResponse};

#[path = "./db/mongo.rs"]
mod mongo;
use mongo::DB;


mod structs;

use structs::data::UserData;

mod db_services;
use db_services::user_service::user::user_service_client;
use db_services::user_service::user::UserRequest;


async fn user_exists (user: UserRequest) -> bool {

    let db = DB::init().await;

    let user = db.find_user(user).await.unwrap();

    if user.is_some() {
        return true;
    } else {
        return false;
    }

} 


async fn db_create (data: web::Json<UserData>) -> impl Responder {
    let user = data.into_inner();
    let user = UserRequest {
        name: user.name,
        age: user.age,
        email: user.email,
        password: user.password
    };

    let cloned_user = user.clone();

    if user_exists(cloned_user).await {
        return HttpResponse::Ok().json(json!({
            "message": "User already exists"
        }))
    }




    let mut client = user_service_client::UserServiceClient::connect("http://[::1]:50051").await.unwrap();
    let response = client.create_user(user).await.unwrap();


    


    HttpResponse::Ok().json(json!({
        "message": response.into_inner().message
    }))
}



#[actix_web::main]


async fn main () -> std::io::Result<()> {



    println!("Server listening on {}", 8080);

    HttpServer::new(move || {
        App::new()
            .route("/", web::post().to(db_create))
    })
    .bind(("127.0.0.1", 8080))
    ?
    .run()
    .await
}