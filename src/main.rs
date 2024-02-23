
use axum::{
    routing::{get, post},
    Router
};

use std::sync::{Arc, Mutex};

use askama::Template;

use tower_http::add_extension::AddExtensionLayer;


mod cell;
mod config;
mod conway;


use conway::{conways_game_of_life, update_cells, set_preset};


#[derive(Template)]
#[template(path = "page.html")]
struct PageTemplate{
    length: i32,
}

#[tokio::main]
async fn main(){

    //let preset = config::config();
    let preset = "toad";

    let cells = Arc::new(Mutex::new(cell::cell::Cells::new(preset).unwrap()));




    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/conway", get(conways_game_of_life))
        .route("/next_generation", get(update_cells).post(update_cells))
        .route("/set_preset", get(set_preset).post(set_preset))
        .layer(AddExtensionLayer::new(cells));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}


