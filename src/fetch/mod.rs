const API_URL: &str = "https://conduit.productionready.io/api";

mod home;

pub use home::*;
use derive_more as dmore;

#[derive(Debug, dmore::AsMut)]
struct UrlBuilder(String);
#[derive(Debug, dmore::AsMut)]
struct Article(String);
#[derive(Debug, dmore::AsMut)]
struct AnArticle(String);
#[derive(Debug, dmore::AsMut)]
struct ArticleInPage(String);

trait Builder: Sized + AsMut<String> {
    fn filter(&mut self, filter: &str, value: &str) {
        self.as_mut().push_str(filter);
        self.as_mut().push('=');
        self.as_mut().push_str(value);
    }

    fn first_filter(&mut self, filter: &str, value: &str) {
        self.as_mut().push('?');
        self.filter(filter, value);
    }

    fn more_filter(&mut self, filter: &str, value: &str) {
        self.as_mut().push('&');
        self.filter(filter, value);
    }

    fn path(&mut self, path: &str) {
        self.as_mut().push('/');
        self.as_mut().push_str(path);
    }
}

impl Builder for UrlBuilder {}
impl Builder for Article {}
impl Builder for AnArticle {}
impl Builder for ArticleInPage {}

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

    fn _user_profile(&mut self, username: &str) {
        self.path("profiles");
        self.path(username);
    }

    fn user_profile(mut self, username: &str) -> String {
        self._user_profile(username);
        self.0
    }

    fn follow_user(mut self, username: &str) -> String {
        self._user_profile(username);
        self.path("follow");
        self.0
    }

    fn articles(mut self) -> Article {
        self.path("articles");
        Article(self.0)
    }

    fn tags(mut self) -> String {
        self.path("tags");
        self.0
    }
}

impl Article {
    fn page(self, page_number: u32) -> ArticleInPage {
        self.page_with_size(page_number, 10)
    }

    fn page_with_size(mut self, page_number: u32, page_size: u32) -> ArticleInPage {
        self.first_filter("offset", &page_number.to_string());
        self.more_filter("limit", &page_size.to_string());
        ArticleInPage(self.0)
    }

    fn feed_in_page(mut self, page_number: u32) -> String {
        self.path("feed");
        self.page_with_size(page_number, 10).0
    }

    fn feed_in_page_with_size(mut self, page_number: u32, page_size: u32) -> String {
        self.path("feed");
        self.page_with_size(page_number, page_size).0
    }

    fn slug(mut self, slug: &types::Slug) -> AnArticle {
        self.path(&slug);
        AnArticle(self.0)
    }

    fn create_article(self) -> String {
        self.0
    }
}

impl ArticleInPage {
    fn done(self) -> String {
        self.0
    }

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

impl AnArticle {
    fn done(self) -> String {
        self.0
    }

    fn comment(mut self) -> String {
        self.path("comments");
        self.0
    }

    fn delete_comment(mut self, id: u32) -> String {
        self.path("comments");
        self.path(&id.to_string());
        self.0
    }

    fn favorite(mut self) -> String {
        self.path("favorite");
        self.0
    }
}
