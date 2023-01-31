use super::{request_delete, request_get, request_post, Request};

pub fn follow(username: &str) -> Request {
    request_post(&format!("/profiles/{}/follow", username), &())
}

pub fn unfollow(username: &str) -> Request {
    request_delete(&format!("/profiles/{}/follow", username))
}

pub fn get(username: &str) -> Request {
    request_get(&format!("/profiles/{}", username))
}
