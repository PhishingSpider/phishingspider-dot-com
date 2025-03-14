#[macro_use] extern crate rocket;
use rocket::Request;
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        title: "Home"
    })
}

#[catch(404)]
fn four_o_four(req: &Request<'_>) -> Template {
    Template::render("error/404", context! {
        title: "🕸️ 404 - PhishingSpider",
        uri: req.uri()
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index]).register("/", catchers![four_o_four]).attach(Template::fairing())
}



