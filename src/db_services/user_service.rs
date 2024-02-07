use tonic::{ Request, Response, Status};

#[path = "../db/mongo.rs"]
mod mongo;

use mongo::DB;




pub mod user {
    tonic::include_proto!("user");
}


#[derive(Default)]
pub struct MyUserService {}


#[tonic::async_trait]
impl user::user_service_server::UserService for MyUserService{
    async fn create_user(&self, request: Request<user::UserRequest>) -> Result<Response<user::UserResponse>, Status> {
        let user = request.into_inner();


        let db = DB::init().await;

        let user = db.create_user(user).await.unwrap();


        match user {
         
            Ok => {
                let reply = user::UserResponse {
                    message: format!("User created successfully").into(),
                };
                Ok(Response::new(reply))
            }

            Error => {
                let reply = user::UserResponse {
                    message: format!("Failed to create user").into(),
                };
                Ok(Response::new(reply))
            }
        }

       
    }
}
