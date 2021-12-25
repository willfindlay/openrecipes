#[macro_use]
extern crate rocket;

use rocket::fs::relative;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use serde_json::json;

#[get("/")]
fn random() -> Template {
    Template::render("recipe", json!({}))
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("/static")))
        .mount("/", routes![random])
        .attach(Template::fairing())
}
