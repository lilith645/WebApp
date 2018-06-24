use rocket_contrib::{Json, Value as JsonValue};

type ID = usize;

#[derive(Serialize, Deserialize)]
struct Message {
  id: Option<ID>,
  contents: String
}

#[get("/<id>", format = "application/json")]
fn get_id(id: ID) -> Option<Json<Message>> {
  None
}
