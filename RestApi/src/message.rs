use rocket::{Response, State};
//use rocket_cors::Responder;
use rocket_contrib::{Json, Value as JsonValue};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, Guard};
use rocket::response::Responder;

use serde_json;

type ID = usize;

#[derive(Serialize, Deserialize)]
struct Message {
  id: Option<ID>,
  contents: String
}

#[post("/<id>", format="application/json")]
fn new_id(options: State<Cors>, id: ID) -> impl Responder {
  //let response = serde_json::to_string_pretty(&json!({ "status": "ok"})).unwrap();
  let response: String = serde_json::to_string(&(json!({ "status": "ok"}))).unwrap();
  
  options
      .inner()
      .respond_borrowed(|guard| guard.responder(response))
}

#[get("/<id>", format = "application/json")]
fn get_id(id: ID) -> Option<Json<Message>> {
  Some(Json(
    Message {
      id: Some(id),
      contents: String::from("Hello user ".to_string() + &id.to_string()),
    }
  ))
}
