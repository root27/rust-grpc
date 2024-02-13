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

        let user_result = db.create_user(user).await.unwrap();


        Ok(Response::new(user::UserResponse {
            message: "User created successfully".into()
        }))

    }
   
    

    async fn get_user(&self, request: Request<user::GetUserRequest>) -> Result<Response<user::GetUserResponse>, Status> {
        let user = request.into_inner();

        let db = DB::init().await;

        let user = db.get_user(user).await.unwrap();

        match user {
            Some(user) => {
                Ok(Response::new(user::GetUserResponse {
                    id: user.id.unwrap().to_hex(),
                    name: user.name,
                    age: user.age,
                    email: user.email,
                    message: "User found".into()
                }))
            }
            None => {
                Ok(Response::new(user::GetUserResponse {
                    id: "".into(),
                    name: "".into(),
                    age: 0,
                    email: "".into(),
                    message: "User not found".into()
                }))
            }
        }
    }


        async fn update_user(&self, request: Request<user::UpdateUserRequest>) -> Result<Response<user::UpdateUserResponse>, Status> {
            let user = request.into_inner();

            let db = DB::init().await;

            let user = db.update_user(user).await.unwrap();

            match user.upserted_id {
                Some(user) => {
                    Ok(Response::new(user::UpdateUserResponse {
                        message: "User updated
                        successfully".into()
                    }))
                }

                    None => {
                        Ok(Response::new(user::UpdateUserResponse {
                            message: "User not found".into()
                        }))
                    }
                }





        }

}
