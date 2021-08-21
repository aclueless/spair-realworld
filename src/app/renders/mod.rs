use spair::prelude::*;

mod footer;
mod header;

impl spair::Component for super::App {
    type Routes = crate::routes::Route;

    fn init(comp: &spair::Comp<Self>) {
        if let Some(token) = super::get_token() {
            spair::update_component(comp.callback_once_mut(|state| state.get_logged_in_user_info(token)));
        }
    }

    fn render(&self, element: spair::Element<Self>) {
        element
            .render(header::Header)
            .render(&self.route.url())
            .div(|d| match &self.page {
                super::Page::Home(child) => d.component(child),
                super::Page::Register(child) => d.component(child),
                super::Page::Login(child) => d.component(child),
            })
            .render(footer::Footer);
    }
}

impl spair::Application for super::App {
    fn init(comp: &spair::Comp<Self>) -> Self {
        Self::new(comp.clone())
    }

    fn init_router(comp: &spair::Comp<Self>) -> Option<crate::routes::Router> {
        Some(crate::routes::Router {
            app: comp.clone(),
            //home: None,
        })
    }
}
