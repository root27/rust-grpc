use serde::{Serialize,Deserialize};


#[derive(Serialize, Deserialize)]

pub struct Data {
    pub name: String
}


#[derive(Serialize, Deserialize)]

pub struct UserData {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub password: String
}
