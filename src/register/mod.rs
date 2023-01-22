use spair::prelude::*;

mod renders;

pub struct Register {
    set_user_callback: spair::CallbackArg<realworld_shared::types::UserInfoWrapper>,
    register_info: realworld_shared::types::RegisterInfo,
    error: Option<realworld_shared::error::Error>,
}

impl Register {
    fn new(
        set_user_callback: spair::CallbackArg<realworld_shared::types::UserInfoWrapper>,
    ) -> Self {
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
        spair::Future::new(async move {
            realworld_shared::services::auth::register(
                realworld_shared::types::RegisterInfoWrapper {
                    user: self.register_info.clone(),
                },
            )
        })
        .with_fn(|state, r| match r {
            Ok(r) => state.register_ok(r),
            Err(e) => state.register_error(e),
        })
    }

    fn register_ok(&mut self, user: realworld_shared::types::UserInfoWrapper) {
        self.set_user_callback.queue(user)
    }

    fn register_error(&mut self, e: realworld_shared::error::Error) {
        self.error = Some(e.into());
    }
}
