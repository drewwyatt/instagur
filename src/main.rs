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
  let comment = match img.comment {
    Some(inner) => inner.clone(),
    None => String::from(""),
  };
  format!("The url is: '{}', the comment is: '{}'", img.url, comment)
}

fn main() {
  rocket::ignite().mount("/", routes![index, upload]).launch();
}
