#![allow(non_snake_case)]

// Recieve config from front end
// send config to all RTUs, get it back
// send it back to front end
use std::net::SocketAddrV4;

use rocket::{get, routes};
use rocket::http::Method;
use rocket::config::{Config, Environment};
use rocket_contrib::json::{Json};
use rocket_cors::{self, AllowedHeaders, AllowedOrigins};

use reqwest;
use reqwest::header::{HeaderValue, CONTENT_TYPE};

use crate::model::Model;

// Send given model to RTU at given address and returns the updated model.
// This will be called for each RTU.
fn update(current_model: &Model, addr: SocketAddrV4) -> Model {
    let client = reqwest::Client::new();
    let mut body = client
        .post(&format!("http://{}/model", addr))
        // .body(current_model.stringify())
        .body(serde_json::to_string(current_model).expect("Couldn't deserialize model"))
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .send()
        .expect("Something went wrong, could not post to RTU");

    // ...and update current_model with whatever is returned
    let err_msg = format!("Could not get model back from RTU with address {}", addr);
    // Do NOT read body.json() or body.text() before this line, it only works once because crackheads i guess
    // Note: future luke (mildly smarter) knows that calling .text() consumes the body
    serde_json::from_str(&body.text().unwrap()).expect(&err_msg)
}

#[get("/")]
fn index() -> &'static str {
    "BCS Networking Master API, you shouldn't see this"
}

#[get("/running")]
fn running() -> &'static str {
    r#"{"running":"true"}"#
}

/// This is the main endpoint for the client site, it POSTs here
/// and this propogates to all the RTUs in the model
#[post("/model", format = "json", data = "<model>")]
fn propogate_to_RTUs(model: Json<Model>) -> String {
    let mut current_model = model.clone();
    let addrs = current_model.RTUs.iter().map(|rtu| rtu.ipv4 ).collect::<Vec<SocketAddrV4>>();
    for addr in addrs {
        // update the model according to the RTU at that address
        current_model = update(&current_model, addr);
    }
    serde_json::to_string(&current_model).expect("Couldn't serialize model")
}

pub fn run() {
    let config = Config::build(Environment::Development)
        .address("0.0.0.0")
        .port(8000)
        .finalize().unwrap();

    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Content-Type", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors().unwrap();

    let app = rocket::custom(config);

    let routes = routes![index, propogate_to_RTUs, running];
    app
        .mount("/", routes)
        .attach(cors)
        .launch();
}
