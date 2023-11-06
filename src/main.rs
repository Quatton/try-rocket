#[macro_use]
extern crate rocket;

#[get("/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/hello", routes![hello])
}
