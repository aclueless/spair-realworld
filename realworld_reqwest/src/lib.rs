use spair::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    //wasm_logger::init(wasm_logger::Config::default());
    realworld_lib::App::mount_to_body();
}
