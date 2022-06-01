#[macro_use] extern crate rocket;

mod paste_id;
use paste_id::PasteId;

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

            responds with the content of the paste with the given ID.
    "
}

