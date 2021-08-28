mod app;
mod article_editor;
mod error;
mod home;
mod login;
mod register;
mod renders;
mod routes;
mod urls;
mod article_viewer;

use spair::prelude::*;

const REALWORLD_TOKEN_KEY: &str = "realworld-token-key";

fn store_token(token: &str) {
    spair::local_storage()
        .set_item(REALWORLD_TOKEN_KEY, token)
        .expect_throw("Unable to store user token to local storage");
}

fn get_token() -> Option<String> {
    spair::local_storage()
        .get_item(REALWORLD_TOKEN_KEY)
        .expect_throw("Unable to get user token from local storage")
}

trait SetAuthorizationToken {
    fn set_token(self) -> Self;
}

impl SetAuthorizationToken for spair::http::request::Builder {
    fn set_token(self) -> Self {
        if let Some(token) = get_token() {
            self.header("Authorization", format!("Token {}", token))
        } else {
            self
        }
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    wasm_logger::init(wasm_logger::Config::default());
    app::App::mount_to_body();
}
