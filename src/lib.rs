mod app;
mod error;
mod home;
mod routes;
mod urls;

use spair::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    wasm_logger::init(wasm_logger::Config::default());
    app::App::mount_to_body();
}
