use super::{request_get, Request};

/// Get all tags
pub fn get_all() -> Request {
    request_get("/tags")
}
