use rocket::{
    http::Status,
    serde::json::{serde_json::json, Value},
};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> String {
    String::from("Hello world")
}

#[get("/hello/<name>")]
fn greet_name(name: Option<String>) -> String {
    let greeting = match name {
        Some(n) => n,
        None => String::from("No name creature"),
    };

    format!("Hello {}", greeting)
}

struct User {
    name: String,
}

#[get("/json")]
fn get_json() -> Result<Value, Status> {
    Ok(json!({
        "name":"puneet"
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, greet_name, get_json])
}
