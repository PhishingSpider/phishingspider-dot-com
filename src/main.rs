#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "hello phish!!"
}

#[get("/about")]
fn about() -> &'static str {
    "<h1>About PhishingSpider</h1><br>Phishing Spider LLC is a security-conscious tech company based out of Bartlesville, Oklahoma."
}



#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, about])
}


