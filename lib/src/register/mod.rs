use spair::prelude::*;

mod renders;

pub struct Register {
    comp: spair::Comp<Self>,
    set_user_callback: spair::CallbackArg<types::UserInfoWrapper>,
    register_info: types::RegisterInfo,
    error: Option<services::error::Error>,
}

impl Register {
    fn new(
        comp: spair::Comp<Self>,
        set_user_callback: spair::CallbackArg<types::UserInfoWrapper>,
    ) -> Self {
        Self {
            comp,
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

    fn send_register_request(&mut self) {
        self.error = None;
        let data = types::RegisterInfoWrapper {
            user: self.register_info.clone(),
        };
        let cb = self.comp.callback_arg_mut(|state: &mut Self, r| match r {
            Ok(r) => state.register_ok(r),
            Err(e) => state.register_error(e),
        });
        services::auth::register(&data).send().spawn_local_with(cb);
    }

    fn register_ok(&mut self, user: types::UserInfoWrapper) {
        self.set_user_callback.queue(user)
    }

    fn register_error(&mut self, e: services::error::Error) {
        self.error = Some(e);
    }
}
