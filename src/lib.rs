mod app;
mod components;
mod error;
mod routes;

use spair::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    app::App::mount_to_body();
}
