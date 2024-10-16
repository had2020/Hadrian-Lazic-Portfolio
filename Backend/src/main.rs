#[macro_use]
extern crate rocket;

use rocket::http::Status; // for returning no content / Status
use rocket::response::Redirect;
use rocket::http::uri::Origin;
use rocket::serde::json::{json, Value};

/* String
#[get("/")]
fn index() -> String {
    String::from("Hello World!")
}
*/

const TAURI_RELEASES_PREFIX: Origin<'static> = uri!("/tauri-releases");

#[get("/")]
fn index() -> Redirect {
    let msg: Option<&str> = None;
    Redirect::to(uri!(TAURI_RELEASES_PREFIX,google_keep_desktop_api("windows-x86_64", "v1.0.14", msg)))
}

#[get("/google-keep-desktop/<_platform>/<current_version>?<msg>")]
fn google_keep_desktop_api(_platform: &str, current_version: &str, msg: Option<&str>) -> Result<Value, Status> { // can return-> Value or Status as well
    // Status::NoContent
    // error prone logic -> Option / Result
    if let Some(msg) = msg { // if msg contatins something
        println!("{msg}"); // needs to remain in format
        format!("{msg}"); // same thing
        return Err(Status::NoContent); // can be seen in network panel
    }

    Ok(json!({ // the Ok means correct value
        "notes": "IT WORKS"
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount(TAURI_RELEASES_PREFIX, routes![google_keep_desktop_api])
}
