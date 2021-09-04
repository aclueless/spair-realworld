use crate::SetAuthorizationToken;
use spair::prelude::*;

mod renders;

pub struct Settings {
    app_comp: spair::Comp<crate::app::App>,
    user_info: Option<types::UserInfo>,
    user_update_info: types::UserUpdateInfo,
    new_password: String,
    error: Option<crate::error::Error>,
}

impl Settings {
    fn new(app_comp: spair::Comp<crate::app::App>, user_info: Option<types::UserInfo>) -> Self {
        Self {
            app_comp,
            user_info,
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
        let cb = self.app_comp.callback_once_mut(crate::app::App::logout);
        spair::update_component(cb);
    }

    fn request_update_user_info(&self) -> spair::Command<Self> {
        let mut data = types::UserUpdateInfoWrapper {
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

    fn set_user_info(&mut self, user_info: types::UserInfoWrapper) {
        self.user_info = Some(user_info.user);
    }

    fn responsed_error(&mut self, error: spair::ResponsedError<types::ErrorInfo>) {
        self.error = Some(error.into());
    }
}
