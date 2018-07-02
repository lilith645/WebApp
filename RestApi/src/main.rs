#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rand;
extern crate rocket;
extern crate rocket_cors;
extern crate rusqlite;

extern crate serde;
extern crate serde_json;

extern crate validator;

#[macro_use] extern crate validator_derive;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod message;
mod database;
mod profiles;

use database::DbConn;

use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
use std::io;

use rocket::http::RawStr;
use rocket::http::Method;
use rocket::request::Form;
use rocket::{Response, State};
use rocket::response::content;
use rocket::http::ContentType;
use rocket::response::NamedFile;
use rocket::response::Responder;
use rocket::response::content::Content;

use rocket_contrib::{Json, Value as JsonValue};

use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors};

#[get("/api")]
fn borrowed(options: State<Cors>) -> impl Responder {
  let mut rng: rand::ThreadRng = thread_rng();
  let code = ((rng.gen::<f32>()*(8999.0)).floor() as i32 + 1000);
  let temp = code.to_string().clone();
  
  options
      .inner()
      .respond_borrowed(|guard| guard.responder(temp))
}

#[get("/db")]
fn db_test<'a, 'b>(options: State<'a, Cors>, db_conn: State<'b, DbConn>) -> impl Responder<'a> {
  let response: String = db_conn.lock()
    .expect("db connect lock")
    .query_row("SELECT name FROM entries WHERE id = 0", &[], |row| { row.get(0) }).unwrap();
  
  options.inner()
    .respond_borrowed(|guard| guard.responder(response))
}

#[error(404)]
fn not_found() -> Json<JsonValue> {
    Json(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}

fn cors_options() -> Cors {
    let (allowed_origins, failed_origins) = AllowedOrigins::some(&["http://localhost:3000/"]);
    assert!(failed_origins.is_empty());

    // You can also deserialize this
    rocket_cors::Cors {
        allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
}

fn main() {
  let conn = database::open_db_in_memory();
  
  database::init_database(&conn);
  
  rocket::ignite()
  .mount("/", routes![borrowed, db_test, profiles::post_users],)
  .mount("/", rocket_cors::catch_all_options_routes())
  .catch(errors![not_found])
  .manage(Mutex::new(conn))
  .manage(cors_options())
  .launch();
}
