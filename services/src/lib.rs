//! Api requests via yew FetchService

// This sub-crate is extracted from https://github.com/jetli/rust-yew-realworld-example-app.
// The exact commit that I copy from is https://github.com/jetli/rust-yew-realworld-example-app/commit/3a767a98600ee89e8b79fb23dcd9a7539ed59f36

// I planed to use this crate without changes but eventually modified it to experiment with gloo-net

// Use dotenvy_macro instead of dotenv_codegen
use dotenvy_macro::dotenv;

use gloo_storage::{LocalStorage, Storage};
use lazy_static::lazy_static;
use parking_lot::RwLock;

pub mod articles;
pub mod auth;
pub mod comments;
pub mod error;
pub mod profiles;
pub mod tags;

#[cfg(feature = "reqwest")]
mod reqwest;
#[cfg(feature = "reqwest")]
pub use self::reqwest::{request_delete, request_get, request_post, request_put, Request};

#[cfg(feature = "gloo-net")]
mod gloo_net;
#[cfg(feature = "gloo-net")]
pub use self::gloo_net::{request_delete, request_get, request_post, request_put, Request};

// Make sure the value of API_ROOT is correctly set in `crate-root/.env`
const API_ROOT: &str = dotenv!("API_ROOT");
// Declaration of TOKEN_KEY is removed from here

lazy_static! {
    /// Jwt token read from local storage.
    pub static ref TOKEN: RwLock<Option<String>> = RwLock::new(None);
    // Always set to None.
    // Code to read the token from LocalStorage removed.
}

/// Set jwt token to local storage.
pub fn set_token(key: &str, token: Option<&str>) {
    // Required the storage key to be passed in (originaly used const TOKEN_KEY)
    if let Some(t) = token {
        LocalStorage::set(key, t).expect("failed to set");
    } else {
        LocalStorage::delete(key);
    }
    let mut token_lock = TOKEN.write();
    *token_lock = token.map(ToString::to_string);
}

/// Get jwt token from lazy static.
pub fn get_token() -> Option<String> {
    let token_lock = TOKEN.read();
    token_lock.clone()
}

/// Set limit for pagination
fn limit(count: u32, p: u32) -> String {
    let offset = if p > 0 { p * count } else { 0 };
    format!("limit={}&offset={}", count, offset)
}
