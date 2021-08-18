mod renders;

pub struct Register {
    register_info: types::RegisterInfo,
    error: Option<crate::error::Error>,
}

impl Register {
    fn new() -> Self {
        Self {
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

    fn send_register_request(&self) -> spair::Command<Self> {
        spair::Request::post()
    }
}
