use super::{request_get, request_post, request_put, Request};

/// Get current user info
pub fn current() -> Request {
    request_get("/user")
}

/// Login a user
pub fn login(login_info: &types::LoginInfoWrapper) -> Request {
    request_post("/users/login", login_info)
}

/// Register a new user
pub fn register(register_info: &types::RegisterInfoWrapper) -> Request {
    request_post("/users", register_info)
}

/// Save info of current user
pub fn save(user_update_info: &types::UserUpdateInfoWrapper) -> Request {
    request_put("/user", user_update_info)
}
