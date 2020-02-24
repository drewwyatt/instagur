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
  let img_inner = img.into_inner();
  let comment = img_inner.comment.unwrap_or("".into());
  format!("The url is: '{}', the comment is: '{}'", img_inner.url, comment)
}

fn main() {
  rocket::ignite().mount("/", routes![index, upload]).launch();
}
