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
}
