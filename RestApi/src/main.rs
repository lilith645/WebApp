#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_cors;

#[macro_use] extern crate serde_derive;

use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors};

use std::collections::HashMap;

use rocket::{Response, State};

use rocket::http::Method;

use rocket::response::content;
use rocket::response::Responder;
use rocket::response::content::Content;
use rocket::http::ContentType;

use std::path::PathBuf;
use std::io;
use rocket::request::Form;
use rocket::http::RawStr;
use rocket::response::NamedFile;
use rocket::response::content::Json;

#[get("/hello/<name>/<age>/<cool>")]
fn cool(name: String, age: u8, cool: bool) -> String {
  if cool {
    format!("You are a cool {} year old, {}!", age, name)
  } else {
    format!("{}, we need to talk about your coolness", name)
  }
}

#[get("/hello/<name>")]
fn hello(name: &RawStr) -> String {
  format!("Hello, {}!", name.as_str())
}

#[get("/world")]
fn world() -> &'static str {
  "Hello world!"
}


#[derive(FromForm)]
struct UserLogin<'r> {
  username: &'r RawStr,
  password: &'r RawStr,
}

#[post("/login", data = "<user_form>")]
fn login<'a>(user_form: Form<'a, UserLogin<'a>>) -> String {
  let user = user_form.get();
  
  format!("Thanks for logging in {}", user.username.as_str())
}

/*
#[derive(Serialize)]
struct TemplateContext {
    name: String,
    items: Vec<String>
}

#[get("/hello/<name>")]
fn get(name: String) -> Template {
    let context = TemplateContext {
        name: name,
        items: vec!["One", "Two", "Three"].iter().map(|s| s.to_string()).collect()
    };

    Template::render("index", &context)
}*/

#[get("/api")]
fn borrowed(options: State<Cors>) -> impl Responder {
    options
        .inner()
        .respond_borrowed(|guard| guard.responder("8749"))
}

fn cors_options() -> Cors {
    let (allowed_origins, failed_origins) = AllowedOrigins::some(&["http://localhost:3000/"]);
    assert!(failed_origins.is_empty());

    // You can also deserialize this
    rocket_cors::Cors {
        allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
}

fn main() {
  rocket::ignite()
  .mount("/", routes![borrowed],)
  .mount("/", rocket_cors::catch_all_options_routes())
  .manage(cors_options())
  .launch();
}
