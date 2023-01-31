use spair::prelude::*;

mod renders;

pub struct Login {
    comp: spair::Comp<Self>,
    set_user_callback: spair::CallbackArg<types::UserInfoWrapper>,
    login_info: types::LoginInfo,
    error: Option<services::error::Error>,
}

impl Login {
    fn new(
        comp: spair::Comp<Self>,
        set_user_callback: spair::CallbackArg<types::UserInfoWrapper>,
    ) -> Self {
        Self {
            comp,
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

    fn send_login_request(&mut self) {
        self.error = None;
        let login_info = self.login_info.clone();
        let cb = self.comp.callback_arg_mut(|state: &mut Self, lr| match lr {
            Ok(lr) => state.login_ok(lr),
            Err(e) => state.login_error(e),
        });
        services::auth::login(&types::LoginInfoWrapper { user: login_info })
            .send()
            .spawn_local_with(cb);
    }

    fn login_ok(&mut self, user: types::UserInfoWrapper) {
        self.set_user_callback.queue(user);
    }

    fn login_error(&mut self, e: services::error::Error) {
        self.error = Some(e);
    }
}
