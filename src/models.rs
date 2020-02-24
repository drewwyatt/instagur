use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ImgurImg {
  pub comment: Option<String>,
  pub url: String,
}
