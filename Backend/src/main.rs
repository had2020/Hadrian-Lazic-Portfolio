#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::response::Redirect;

/* String
#[get("/")]
fn index() -> String {
    String::from("Hello World!")
}
*/

#[get("/")]
fn index() -> Redirect {
    let msg: Option<&str> = None;
    Redirect::to(uri!(google_keep_desktop_api(
        "windows-x86_64",
        "v1.0.14",
        msg
    )))
}

#[get("/google-keep-desktop/<_platform>/<current_version>?<msg>")]
fn google_keep_desktop_api(_platform: &str, current_version: &str, msg: Option<&str>) -> Status {
    Status::NoContent
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/tauri-releases", routes![google_keep_desktop_api])
}
