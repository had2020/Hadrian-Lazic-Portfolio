// rocket v0.5.0-rc.1
use rocket::{
    self,
    serde::{json::Json, Deserialize, Serialize},
};

#[derive(Deserialize, Serialize)]
struct User {
    name: String,
    age: u8,
}

#[rocket::post("/post", format = "json", data = "<user>")]
fn post_data(user: Json<User>) -> Json<User> {
    let name: String = user.name.clone();
    let age: u8 = user.age.clone();

    Json(User { name, age })
}

#[rocket::main]
async fn main() {
    if let Err(err) = rocket::build()
        .mount("/", rocket::routes![post_data])
        .launch()
        .await
    {
        println!("Rocket Rust couldn't take off successfully!");
        drop(err); // Drop initiates Rocket-formatted panic
    }
}
