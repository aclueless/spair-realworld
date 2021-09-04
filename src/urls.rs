//const API_URL: &str = "https://conduit.productionready.io/api";
const API_URL: &str = "http://localhost:8000/api";

use derive_more as dmore;

#[derive(Debug, dmore::AsMut)]
pub struct UrlBuilder(String);
#[derive(Debug, dmore::AsMut)]
pub struct Profile(String);
#[derive(Debug, dmore::AsMut)]
pub struct Articles(String);
#[derive(Debug, dmore::AsMut)]
pub struct TheSpecifiedArticle(String);
#[derive(Debug, dmore::AsMut)]
pub struct ArticlesInPage(String);

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
impl Builder for Profile {}
impl Builder for Articles {}
impl Builder for TheSpecifiedArticle {}
impl Builder for ArticlesInPage {}

impl UrlBuilder {
    pub fn new() -> Self {
        Self(API_URL.to_string())
    }

    pub fn login(mut self) -> String {
        self.path("users");
        self.path("login");
        self.0
    }

    pub fn register_user(mut self) -> String {
        self.path("users");
        self.0
    }

    pub fn user(mut self) -> String {
        self.path("user");
        self.0
    }

    pub fn profile(mut self, username: &str) -> Profile {
        self.path("profiles");
        self.path(username);
        Profile(self.0)
    }

    pub fn articles(mut self) -> Articles {
        self.path("articles");
        Articles(self.0)
    }

    pub fn tags(mut self) -> String {
        self.path("tags");
        self.0
    }
}

impl Profile {
    pub fn get(self) -> String {
        self.0
    }

    pub fn follow(mut self) -> String {
        self.path("follow");
        self.0
    }
}

impl Articles {
    pub fn page(self, page_number: u32) -> ArticlesInPage {
        self.page_with_size(page_number, crate::ARTICLES_PER_PAGE)
    }

    pub fn page_with_size(mut self, page_number: u32, page_size: u32) -> ArticlesInPage {
        self.first_filter("offset", &page_number.to_string());
        self.more_filter("limit", &page_size.to_string());
        ArticlesInPage(self.0)
    }

    pub fn feed_in_page(self, page_number: u32) -> String {
        self.feed_in_page_with_size(page_number, crate::ARTICLES_PER_PAGE)
    }

    pub fn feed_in_page_with_size(mut self, page_number: u32, page_size: u32) -> String {
        self.path("feed");
        self.page_with_size(page_number, page_size).0
    }

    pub fn slug(mut self, slug: &types::Slug) -> TheSpecifiedArticle {
        self.path(&slug);
        TheSpecifiedArticle(self.0)
    }

    pub fn create_article(self) -> String {
        self.0
    }
}

impl ArticlesInPage {
    pub fn done(self) -> String {
        self.0
    }

    pub fn tag(mut self, tag: &str) -> String {
        self.filter("tag", tag);
        self.0
    }

    pub fn author(mut self, author: &str) -> String {
        self.filter("author", author);
        self.0
    }

    pub fn favorited_by(mut self, user: &str) -> String {
        self.filter("favorited", user);
        self.0
    }
}

impl TheSpecifiedArticle {
    pub fn get(self) -> String {
        self.0
    }

    pub fn update(self) -> String {
        self.0
    }

    pub fn delete(self) -> String {
        self.0
    }

    pub fn comment(mut self) -> String {
        self.path("comments");
        self.0
    }

    pub fn delete_comment(mut self, id: u32) -> String {
        self.path("comments");
        self.path(&id.to_string());
        self.0
    }

    pub fn favorite(mut self) -> String {
        self.path("favorite");
        self.0
    }
}
