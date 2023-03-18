use actix_web::{get, post, web::Json, HttpResponse, Responder, ResponseError};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    username: String,
    password: String,
}

#[post("/users")]
pub async fn create_user(user: Json<User>) -> impl Responder {
    println!("{:?}", user.into_inner());

    HttpResponse::Ok()
}
