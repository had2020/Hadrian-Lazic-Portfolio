#[macro_use]
extern crate rocket;
use rocket::serde::{json::Json, Serialize};

struct Thing {
    id: usize,
    name: String,
    description: String,
}

#[get("/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/api/<id>", format = "json")]
fn api(id: usize, name: &str, description: &str) -> Json<Thing> {
    Json(Thing {
        id,
        name: "Hadrian".to_string(),
        description: "Dude".to_string(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/hello", routes![hello])
}
