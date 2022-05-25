#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

#[get("/")]
fn index() -> &'static str {
    "
    USAGE
        POST /

            accepts raw data in the body of the request and responds with a URL of a page containing the body's content.

        GET /<id>

            responds with the content of the page with the given ID.
    "
}

