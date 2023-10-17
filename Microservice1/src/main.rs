mod routes;
mod models;

#[macro_use]
extern crate rocket;


// fn main() {
//     println!("Hello, world!");
// }


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}



