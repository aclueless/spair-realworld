const API_URL: &str = "https://conduit.productionready.io/api";

mod home;

pub use home::*;

#[derive(Debug, newtype::NewType)]
struct UrlBuilder(String);
#[derive(Debug, newtype::NewType)]
struct Article(String);
#[derive(Debug, newtype::NewType)]
struct ArticleList(String);

impl UrlBuilder {
    fn new() -> Self {
        Self (API_URL.to_string())
    }

    fn article(mut self) -> Article {
        self.0.push_str("/article");
        Article(self.0)
    }
}

impl Article {
    fn page(mut self, page_number: u32) -> ArticleList {
        self.page_with_size(page_number, 10)
    }

    fn page_with_size(mut self, page_number: u32, page_size: u32) -> ArticleList {
        self.0.push_str("?offset=");
        self.0.push_str(&page_number.to_string());
        self.0.push_str("&limit=");
        self.0.push_str(&page_size.to_string());
        ArticleList(self.0)
    }
}
