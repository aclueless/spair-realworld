use spair::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    //wasm_logger::init(wasm_logger::Config::default());
    lib::App::mount_to_body();
}
