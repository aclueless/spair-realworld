use super::{limit, request_delete, request_get, request_post, request_put, Request};

/// Get all articles
pub fn all(page: u32) -> Request {
    request_get(&format!("/articles?{}", limit(10, page)))
}

/// Get articles filtered by author
pub fn by_author(author: &str, page: u32) -> Request {
    request_get(&format!("/articles?author={}&{}", author, limit(10, page)))
}

/// Get articles filtered by tag
pub fn by_tag(tag: &str, page: u32) -> Request {
    request_get(&format!("/articles?tag={}&{}", tag, limit(10, page)))
}

/// Delete an article
pub fn del(slug: &str) -> Request {
    request_delete(&format!("/articles/{}", slug))
}

/// Favorite an article
pub fn favorite(slug: &str) -> Request {
    request_post(&format!("/articles/{}/favorite", slug), &())
}

/// Unfavorite an article
pub fn unfavorite(slug: &str) -> Request {
    request_delete(&format!("/articles/{}/favorite", slug))
}

/// Get articles favorited by an author
pub fn favorited_by(author: &str, page: u32) -> Request {
    request_get(&format!(
        "/articles?favorited={}&{}",
        author,
        limit(10, page)
    ))
}

/// Get feed of articles
pub fn feed() -> Request {
    request_get(&format!("/articles/feed?{}", limit(10, 0)))
}

/// Get an article
pub fn get(slug: &str) -> Request {
    request_get(&format!("/articles/{}", slug))
}

/// Update an article
pub fn update(slug: &str, article: &types::ArticleCreateUpdateInfoWrapper) -> Request {
    request_put(&format!("/articles/{}", slug), article)
}

/// Create an article
pub fn create(article: &types::ArticleCreateUpdateInfoWrapper) -> Request {
    request_post("/articles", article)
}
