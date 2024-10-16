#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> String{
    String::from("Hello World!")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
}
