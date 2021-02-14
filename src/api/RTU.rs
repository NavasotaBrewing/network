// RTU API
//
// Recieve config from front end
// send config to all RTUs, get it back
// send it back to front end
use rocket_contrib::json::Json;
use rocket::config::{Config, Environment};
use rocket_cors;
use rocket::http::Method;
use rocket::{get, routes};
use rocket_cors::{AllowedHeaders, AllowedOrigins};

use crate::model::Model;

#[get("/")]
fn index() -> &'static str {
    "BCS Networking RTU API, you definitely shouldn't see this"
}

#[get("/running")]
fn running() -> &'static str {
    r#"{"running":"true"}"#
}

#[post("/model", format = "json", data = "<model>")]
fn update_model(model: Json<Model>) -> String {
    // Receive a model, consume the Json wrapper, as one does
    let model = model.into_inner();
    // Update the model
    let updated_model = Model::update(&model, &model.mode);
    // Return to sender
    serde_json::to_string(&updated_model).expect("Couldn't serialize model")
}

pub fn run() {

    let config = Config::build(Environment::Development)
        .address("0.0.0.0")
        .port(3012)
        .finalize().unwrap();

    let app = rocket::custom(config);

    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Content-Type", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors().unwrap();


    let routes = routes![index, update_model, running];
    app
        .mount("/", routes)
        .attach(cors)
        .launch();
}
