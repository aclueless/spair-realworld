use spair::prelude::*;

mod renders;

pub struct Register {
    set_user_callback: spair::CallbackArg<types::UserInfoWrapper>,
    register_info: types::RegisterInfo,
    error: Option<crate::error::Error>,
}

impl Register {
    fn new(set_user_callback: spair::CallbackArg<types::UserInfoWrapper>) -> Self {
        Self {
            set_user_callback,
            register_info: Default::default(),
            error: None,
        }
    }

    fn set_username(&mut self, username: String) {
        self.register_info.username = username;
    }

    fn set_email(&mut self, email: String) {
        self.register_info.email = email;
    }

    fn set_password(&mut self, password: String) {
        self.register_info.password = password;
    }

    fn send_register_request(&mut self) -> spair::Command<Self> {
        self.error = None;
        let url = crate::urls::UrlBuilder::new().register_user();
        spair::http::Request::post(&url)
            .text_mode()
            .body()
            .json(&types::RegisterInfoWrapper {
                user: self.register_info.clone(),
            })
            .response()
            .json(Self::register_ok, Self::register_error)
    }

    fn register_ok(&mut self, user: types::UserInfoWrapper) {
        self.set_user_callback.queue(user)
    }

    fn register_error(&mut self, e: spair::ResponsedError<types::ErrorInfo>) {
        self.error = Some(e.into());
    }
}
