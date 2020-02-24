#![feature(proc_macro_hygiene, decl_macro)]
use rocket_contrib::json::Json;

mod models;
use models::ImgurImg;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
  "Hello, worldd!"
}

#[post("/image", format = "json", data = "<img>")]
fn upload(img: Json<ImgurImg>) -> String {
  img.url.clone()
}

fn main() {
  rocket::ignite().mount("/", routes![index, upload]).launch();
}
