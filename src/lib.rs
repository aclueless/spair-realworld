mod app;
mod article_editor;
mod article_list;
mod article_viewer;
mod error;
mod home;
mod login;
mod profile;
mod register;
mod routes;
mod settings;

use spair::prelude::*;

const LOCAL_STORAGE_TOKEN_KEY: &str = "spair-realworld-jwt-token-key";

#[wasm_bindgen(start)]
pub fn start() {
    wasm_logger::init(wasm_logger::Config::default());
    app::App::mount_to_body();
}
