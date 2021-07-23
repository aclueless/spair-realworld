const API_URL: &str = "https://conduit.productionready.io/api";

mod home;

pub use home::*;
use derive_more as dmore;

#[derive(Debug, dmore::AsMut)]
struct UrlBuilder(String);
#[derive(Debug, dmore::Into, dmore::AsMut)]
struct Article(String);
#[derive(Debug, dmore::Into, dmore::AsMut)]
struct No(String);

trait Builder: Sized + AsMut<String> {
    fn filter(&mut self, filter: &str, value: &str) {
        self.as_mut().push_str(filter);
        self.as_mut().push_str("=");
        self.as_mut().push_str(value);
    }

    fn first_filter(&mut self, filter: &str, value: &str) {
        self.as_mut().push_str("?");
        self.filter(filter, value);
    }

    fn more_filter(&mut self, filter: &str, value: &str) {
        self.as_mut().push_str("&");
        self.filter(filter, value);
    }

    fn path(&mut self, path: &str) {
        self.as_mut().push_str("/");
        self.as_mut().push_str(path);
    }
}

impl Builder for UrlBuilder {}
impl Builder for Article {}

impl UrlBuilder {
    fn new() -> Self {
        Self (API_URL.to_string())
    }

    fn login(mut self) -> String {
        self.path("users");
        self.path("login");
        self.0
    }

    fn register_user(mut self) -> String {
        self.path("users");
        self.0
    }

    fn user(mut self) -> String {
        self.path("user");
        self.0
    }

    fn user_profile(mut self, username: &str) -> String {
        self.path("profiles");
        self.path(username);
        self.0
    }

    fn follow_user(mut self, username: &str) -> String {
        self.path("profiles");
        self.path(username);
        self.path("follow");
        self.0
    }

    fn articles_in_page(self, page_number: u32) -> Article {
        self.articles_in_page_with_size(page_number, 10)
    }

    fn articles_in_page_with_size(mut self, page_number: u32, page_size: u32) -> Article {
        self.path("articles");
        self.first_filter("offset", &page_number.to_string());
        self.more_filter("limit", &page_size.to_string());
        Article(self.0)
    }

    fn tags(mut self) -> String {
        self.path("tags");
        self.0
    }
}

impl Article {
    fn tag(mut self, tag: &str) -> String {
        self.filter("tag", tag);
        self.0
    }

    fn author(mut self, author: &str) -> String {
        self.filter("author", author);
        self.0
    }

    fn favorited(mut self, favorited: &str) -> String {
        self.filter("favorited", favorited);
        self.0
    }
}
