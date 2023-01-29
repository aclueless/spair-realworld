use spair::prelude::*;

mod renders;

pub struct Settings {
    comp: spair::Comp<Self>,
    props: Props,
    user_update_info: realworld_shared::types::UserUpdateInfo,
    new_password: String,
    error: Option<realworld_shared::error::Error>,
}

pub struct Props {
    pub logout_callback: spair::Callback,
    pub set_user_callback: spair::CallbackArg<realworld_shared::types::UserInfoWrapper>,
    pub user_info: Option<realworld_shared::types::UserInfo>,
}

impl Settings {
    fn new(comp: spair::Comp<Self>, props: Props) -> Self {
        Self {
            comp,
            props,
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
        self.props.logout_callback.queue()
    }

    fn request_update_user_info(&self) {
        let mut data = realworld_shared::types::UserUpdateInfoWrapper {
            user: self.user_update_info.clone(),
        };
        if !self.new_password.is_empty() {
            data.user.password = Some(self.new_password.clone());
        }
        let cb = self.comp.callback_arg_mut(|state: &mut Self, u| match u {
            Ok(u) => state.set_user_info(u),
            Err(e) => state.responsed_error(e),
        });

        realworld_shared::services::auth::save(data).spawn_local_with(cb);
    }

    fn set_user_info(&mut self, user_info: realworld_shared::types::UserInfoWrapper) {
        self.props.user_info = Some(user_info.user.clone());
        self.props.set_user_callback.queue(user_info);
    }

    fn responsed_error(&mut self, error: realworld_shared::error::Error) {
        self.error = Some(error);
    }
}
