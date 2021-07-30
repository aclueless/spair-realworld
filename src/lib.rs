mod app;
mod error;
mod pages;
mod renders;
mod routes;
mod fetch;

use spair::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    wasm_logger::init(wasm_logger::Config::default());
    app::App::mount_to_body();
}
