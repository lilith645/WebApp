use rocket::{Response, State};
//use rocket_cors::Responder;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, Guard};
use rocket::response::Responder;

use validator::{Validate, ValidationError, ValidationErrors};
use rocket_contrib::{Json, Value};
use serde_json;

#[derive(Deserialize, Validate)]
struct NewUserData {
  #[validate(length(min="1"))]
  username: Option<String>,
  #[validate(email)]
  email: Option<String>,
  #[validate(length(min="8"))]
  password: Option<String>,
}

#[derive(Deserialize)]
pub struct NewUser {
  user: NewUserData,
}

#[post("/users", format="multipart/form-data", data="<new_user>")]
pub fn post_users(new_user: String) -> Json<Value> {
  let line = new_user.trim();
  let line = line.trim_left();
  let line = line.trim_matches('\t');
  
  let bracket_offset = line.find('{').unwrap_or(line.len());
  let mut data = line.to_string();
  
  let (trash, data)= data.split_at(bracket_offset);
  
  let bracket_offset = data.rfind('}').unwrap_or(0);
  let (data, trash) = data.split_at(bracket_offset);
  let data = data.to_owned() + "}";
  
  let text = "{\"username\":\"dsfg\",\"email\":\"Test@email.com\",\"password\":\"dfgs\"}";
  println!("{}", text);
  let data: NewUserData = serde_json::from_str(&data).unwrap();
  let mut new_user = NewUser {
    user: data
  };
  
  new_user.user.validate();
  
  println!("{:?}", Json(json!({"user": "success" })));
  Json(json!({"user": "success" }))
}

#[get("/profile/<username>")]
fn get_profile<'a, 'b>(options: State<'b, Cors>, username: String) -> impl Responder<'b> {
  let user = username;
  
  let response = format!("Thanks for logging in {}", user);
  options
      .inner()
      .respond_borrowed(|guard| guard.responder(response))
}

#[post("/profile/<username>/follow")]
fn post_profile<'a, 'b>(options: State<'b, Cors>, username: String) -> impl Responder<'b> {
  let user = username;
  
  let response = format!("This is {} profile", user);
  options
      .inner()
      .respond_borrowed(|guard| guard.responder(response))
}
