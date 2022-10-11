use std::net::SocketAddrV4;
use reqwest::{header::{HeaderValue, CONTENT_TYPE}, Method};
use warp::Filter;
use crate::model::Model;

// Recieve config from front end
// send config to all RTUs, get it back
// send it back to front end

// Send given model to RTU at given address and returns the updated model.
// This will be called for each RTU.
fn update(current_model: &Model, addr: SocketAddrV4) -> Model {
    let client = reqwest::blocking::Client::new();
    let body = client
        .post(&format!("http://{}/model", addr))
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

pub async fn run() {
    // let cors = warp::cors()
    //     .allow_any_origin()
    //     .allow_header("content-type")
    //     .allow_headers(vec![
    //         "Access-Control-Allow-Origin"
    //     ])
    //     // .allow_headers(
    //     //     vec![
    //     //         "Accept",
    //     //         "Accept-Encoding",
    //     //         "Accept-Language",
    //     //         "Connection",
    //     //         "Content-Length",
    //     //         "Host",
    //     //         "Sec-Fetch-Dest",
    //     //         "Sec-Fetch-Site",
    //     //         "content-type",
    //     //         "User-Agent",
    //     //         "Sec-Fetch-Mode",
    //     //         "Referer",
    //     //         "Origin",
    //     //         "Access-Control-Request-Method",
    //     //         "Access-Control-Request-Headers",
    //     //         "Access-Control-Allow-Origin",
    //     //     ]
    //     // )
    //     .allow_methods(vec!["GET", "POST"]);
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::POST])
        .allow_header("content-type");

    let running = warp::path("running").map(|| r#"{"running":"true"}"# ).with(&cors);
    
    let config_dep = warp::path("configuration")
        .map(|| {
            "deprecated"
        })
        .with(&cors);

    let handle_model = warp::path("model")
        .and(warp::body::json())
        .map(|model: Model| {
            // println!("Master API recieved a model, about to send it to the RTUs");
            let addrs: Vec<SocketAddrV4> = model
                .RTUs
                .iter()
                .map(|rtu| rtu.ipv4 )
                .collect();

            let mut current_model = model;
            for addr in addrs {
                current_model = update(&current_model, addr);
            }
            
            serde_json::to_string(&current_model).expect("Couldn't serialize model")
        })
        .with(&cors);

    let routes = running
        .or(config_dep)
        .or(handle_model);

        warp::serve(routes)
            .run(([0, 0, 0, 0], 8000))
            .await;
}