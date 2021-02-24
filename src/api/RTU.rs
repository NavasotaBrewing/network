use std::convert::Infallible;

use warp::Filter;
use crate::model::Model;


pub async fn run() {

    let running = warp::path("running").map(|| r#"{"running":"true"}"# );
    
    let handle_model_route = warp::path("model")
        .and(warp::body::json())
        .and_then(handle_model);

    let routes = running.or(handle_model_route);
    warp::serve(routes)
        .run(([0, 0, 0, 0], 3012))
        .await;
}

async fn handle_model(model: Model) -> Result<impl warp::Reply, Infallible> {
    println!("RTU recieved model, updating and sending it back");
    let updated = Model::update(&model, &model.mode).await;
    Ok(serde_json::to_string(&updated).expect("Couldn't serialize model"))
}