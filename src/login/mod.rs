use spair::prelude::*;

mod renders;

pub struct Login {
    set_user_callback: spair::CallbackArg<types::UserInfoWrapper>,
    login_info: types::LoginInfo,
    error: Option<crate::error::Error>,
}

impl Login {
    fn new(set_user_callback: spair::CallbackArg<types::UserInfoWrapper>) -> Self {
        Self {
            set_user_callback,
            login_info: Default::default(),
            error: None,
        }
    }

    fn set_email(&mut self, email: String) {
        self.login_info.email = email;
    }

    fn set_password(&mut self, password: String) {
        self.login_info.password = password;
    }

    fn send_login_request(&mut self) -> spair::Command<Self> {
        self.error = None;
        let url = crate::urls::UrlBuilder::new().login();
        spair::http::Request::post(&url)
            .text_mode()
            .body()
            .json(&types::LoginInfoWrapper {
                user: self.login_info.clone(),
            })
            .response()
            .json(Self::login_ok, Self::login_error)
    }

    fn login_ok(&mut self, user: types::UserInfoWrapper) {
        self.set_user_callback.queue(user);
    }

    fn login_error(&mut self, e: spair::ResponsedError<types::ErrorInfo>) {
        self.error = Some(e.into());
    }
}
