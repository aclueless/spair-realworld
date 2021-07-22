use spair::prelude::*;

pub struct App {
    pub route: crate::routes::Route,
    pub user: Option<types::UserInfo>,
}

impl App {
    fn new() -> Self {
        Self {
            route: crate::routes::Route::Home,
            user: None,
        }
    }
}

impl spair::Component for App {
    type Routes = crate::routes::Route;

    fn render(&self, element: spair::Element<Self>) {
        element
            .render(crate::renders::header::Header)
            .render(crate::renders::footer::Footer);
    }
}

impl spair::Application for App {
    fn with_comp(_comp: spair::Comp<Self>) -> Self {
        Self::new()
    }
}
