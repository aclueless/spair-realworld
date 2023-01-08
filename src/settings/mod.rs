use crate::SetAuthorizationToken;
use spair::prelude::*;

mod renders;

pub struct Settings {
    logout_callback: spair::Callback,
    user_info: Option<UserInfo>,
    user_update_info: UserUpdateInfo,
    new_password: String,
    error: Option<crate::error::Error>,
}

pub struct Props {
    pub logout_callback: spair::Callback,
    pub user_info: Option<UserInfo>,
}

impl Settings {
    fn new(props: Props) -> Self {
        Self {
            logout_callback: props.logout_callback,
            user_info: props.user_info,
            user_update_info: Default::default(),
            new_password: String::new(),
            error: None,
        }
    }

    fn set_image(&mut self, url: String) {
        self.user_update_info.image = url;
    }

    fn set_username(&mut self, username: String) {
        self.user_update_info.username = username;
    }

    fn set_bio(&mut self, bio: String) {
        self.user_update_info.bio = bio;
    }

    fn set_email(&mut self, email: String) {
        self.user_update_info.email = email;
    }

    fn set_password(&mut self, password: String) {
        self.new_password = password;
    }

    fn is_valid(&self) -> bool {
        false
    }

    fn logout(&self) {
        self.logout_callback.queue()
    }

    fn request_update_user_info(&self) -> spair::Command<Self> {
        let mut data = UserUpdateInfoWrapper {
            user: self.user_update_info.clone(),
        };
        if !self.new_password.is_empty() {
            data.user.password = Some(self.new_password.clone());
        }
        let url = crate::urls::UrlBuilder::new().user();
        spair::http::Request::put(&url)
            .set_token()
            .text_mode()
            .body()
            .json(&data)
            .response()
            .json(Self::set_user_info, Self::responsed_error)
    }

    fn set_user_info(&mut self, user_info: UserInfoWrapper) {
        self.user_info = Some(user_info.user);
    }

    fn responsed_error(&mut self, error: spair::ResponsedError<ErrorInfo>) {
        self.error = Some(error.into());
    }
}
