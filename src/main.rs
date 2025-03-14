#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};
use tera::Tera;

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        title: "Home"
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index]).attach(Template::fairing())
}



