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
        self.0.push_str("/articles");
        Article(self.0)
    }

    fn tags(mut self) -> String {
        self.0.push_str("/tags");
        self.0
    }
}

trait Paged: Sized + std::ops::DerefMut<Target = String> {
    fn page(self, page_number: u32) -> String {
    }
}

impl Article {
    fn page(self, page_number: u32) -> String {
        self.page_with_size(page_number, 10)
    }

    fn page_with_size(mut self, page_number: u32, page_size: u32) -> String {
        self.0.push_str("?offset=");
        self.0.push_str(&page_number.to_string());
        self.0.push_str("&limit=");
        self.0.push_str(&page_size.to_string());
        self.0
    }
}
