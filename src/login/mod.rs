mod renders;

pub struct Login {
    set_user_callback: spair::CallbackArg<realworld_shared::types::UserInfoWrapper>,
    login_info: realworld_shared::types::LoginInfo,
    error: Option<realworld_shared::error::Error>,
}

impl Login {
    fn new(
        set_user_callback: spair::CallbackArg<realworld_shared::types::UserInfoWrapper>,
    ) -> Self {
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
        let login_info = self.login_info.clone();
        spair::Future::new(async move {
            realworld_shared::services::auth::login(realworld_shared::types::LoginInfoWrapper {
                user: login_info,
            })
            .await
        })
        .with_fn(|state: &mut Self, lr| match lr {
            Ok(lr) => state.login_ok(lr),
            Err(e) => state.login_error(e),
        })
    }

    fn login_ok(&mut self, user: realworld_shared::types::UserInfoWrapper) {
        self.set_user_callback.queue(user);
    }

    fn login_error(&mut self, e: realworld_shared::error::Error) {
        self.error = Some(e);
    }
}
