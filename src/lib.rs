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
mod urls;

use spair::prelude::*;

const ARTICLES_PER_PAGE: u32 = 10;
const REALWORLD_TOKEN_KEY: &str = "realworld-token-key";

fn store_token(token: &str) {
    spair::local_storage()
        .set_item(REALWORLD_TOKEN_KEY, token)
        .expect_throw("Unable to store user token to local storage");
}

fn delete_token() {
    spair::local_storage()
        .remove_item(REALWORLD_TOKEN_KEY)
        .expect_throw("Unable to remove user token from local storage");
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

