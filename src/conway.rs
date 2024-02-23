use axum::extract::Form;

use axum::response::Html;
use std::sync::{Arc, Mutex};

use askama::Template;
use axum::Extension;

use crate::cell::cell::Cells;

use serde::Deserialize;

#[derive(Template)]
#[template(path = "page.html")]
struct PageTemplate{
    length: i32,
}


#[derive(Deserialize)]
pub struct Preset{
    preset: String,
}

pub async fn conways_game_of_life(Extension(shared_cell): Extension<Arc<Mutex<Cells>>>) -> Html<String>{
    let shared_cell = shared_cell.lock().unwrap();
    let length = shared_cell.cells.len() as i32;

    let res = PageTemplate{ length: length};

    return Html(res.render().unwrap());
}


pub async fn update_cells(Extension(shared_cell): Extension<Arc<Mutex<Cells>>>) -> Html<String>{
    let mut new_cells = shared_cell.lock().unwrap();
    new_cells.apply_rules();

    let mut squares = String::new();

    squares.push_str("<div class='grid-container'>");
    for row in new_cells.cells.iter() {
        for cell in row.iter() {
            if *cell {
                squares.push_str("<div class='square_alive'></div>");

            } else {
                squares.push_str("<div class='square_dead'></div>");
            }
        }
    }
    squares.push_str("</div>");

    return Html(squares);

}

pub async fn set_preset(Extension(shared_cell): Extension<Arc<Mutex<Cells>>>, form: Form<Preset>) -> Html<String>{
    let preset = form.preset.clone();
    let mut new_cells = shared_cell.lock().unwrap();
    new_cells.get_presets(&preset);
    let mut squares = String::new();

    squares.push_str("<div class='grid-container'>");
    for row in new_cells.cells.iter() {
        for cell in row.iter() {
            if *cell {
                squares.push_str("<div class='square_alive'></div>");

            } else {
                squares.push_str("<div class='square_dead'></div>");
            }
        }
    }

    squares.push_str("</div>");

    return Html(squares);
}

