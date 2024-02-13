use mongodb::{Collection, Client};
use serde::{Deserialize, Serialize};
use tonic::{Request};
use crate::db_services::user_service::user::UserRequest;
use mongodb::results::InsertOneResult;
use mongodb::bson::doc;


#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<mongodb::bson::oid::ObjectId>,
    pub name: String,
    pub age: i32,
    pub email: String,
    pub password: String

}


pub struct DB {
    col: Collection<User>
}




impl DB {
    pub async fn init() -> Self {
        let client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
        let db = client.database("test");
        let col = db.collection("users");
        DB { col }
    }

    pub async fn create_user (&self, user: UserRequest) -> Result<InsertOneResult, mongodb::error::Error> {
        
        

        let new_doc = User {
            id: None,
            name: user.name,
            age: user.age,
            email: user.email,
            password: user.password
        };

        let user = self.
            col.
            insert_one(new_doc, None).await
            .expect("Failed to insert document");

       
        Ok(user)

        }


        pub async fn find_user(&self, user: UserRequest) -> Result<Option<User>, mongodb::error::Error> {
            let user = self.col.find_one(
                Some(doc! {
                    "email": user.email,
                    "password": user.password
                }),
                None
            ).await.unwrap();

            Ok(user)
        }






}

