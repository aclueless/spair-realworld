mod renders;

pub struct Settings {
    user_info: types::UserUpdateInfo,
    password: String,
    error: Option<crate::error::Error>,
}
