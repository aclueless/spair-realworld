use spair::prelude::*;

pub struct App {
    route: Option<crate::routes::AppRoute>,
    user: Option<types::UserInfo>,
}

impl App {
    fn new() -> Self {
        Self {
            route: None,
            user: None,
        }
    }

    pub fn is_at_home(&self) -> bool {
        false
    }

    pub fn is_at_new_post(&self) -> bool {
        false
    }

    pub fn is_at_settings(&self) -> bool {
        false
    }

    pub fn is_at_sign_up(&self) -> bool {
        false
    }
}

impl spair::Component for App {
    type Routes = crate::routes::AppRoute;

    fn render(&self, element: spair::Element<Self>) {
        element
            .render(crate::components::header::Header)
            .render(crate::components::footer::Footer);
    }
}

impl spair::Application for App {
    fn with_comp(_comp: spair::Comp<Self>) -> Self {
        Self::new()
    }
}
