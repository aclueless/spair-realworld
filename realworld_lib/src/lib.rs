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

const LOCAL_STORAGE_TOKEN_KEY: &str = "spair-realworld-jwt-token-key";
const ARTICLES_PER_PAGE: u32 = 10;

pub use app::App;
