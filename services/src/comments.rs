use super::{request_delete, request_get, request_post, Request};

pub fn create(slug: &str, comment: &types::CommentCreateInfoWrapper) -> Request {
    request_post(&format!("/articles/{}/comments", slug), comment)
}

pub fn delete(slug: &str, comment_id: u32) -> Request {
    request_delete(&format!("/articles/{}/comments/{}", slug, comment_id))
}

pub fn for_article(slug: &str) -> Request {
    request_get(&format!("/articles/{}/comments", slug))
}
